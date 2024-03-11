use crate::object::{ContextData, Fault, Method, Nil};
use crate::object::block::Block;
use crate::vm::bytecode::{ByteCode, SpecialInstruction};
use std::sync::{Arc, Mutex, RwLock};

use super::bytecode::Literal;

pub struct Interpreter {
    code: Vec<(usize, Arc<Vec<ByteCode>>)>,
    context: Option<ContextData>,
}

impl Interpreter {
    pub fn new(context: ContextData) -> Self {
        Self {
            code: Vec::new(),
            context: Some(context),
        }
    }
    
    pub fn run_loop(index: usize, interpreters: Arc<RwLock<Vec<Arc<Mutex<Option<Interpreter>>>>>>, lock: Arc<Mutex<()>>) {
        'control: loop {
            let _ = lock.lock();
            let interpreters_ref = interpreters.read().expect("Expected read lock");
            let Some(interpreter) = interpreters_ref.get(index) else {
                continue;
            };
            let interpreter = interpreter.clone();
            let mut interpreter_mut = interpreter.lock().expect("Expected lock");
            drop(interpreters_ref);
            if interpreter_mut.is_none() {
                continue;
            }
            let interpreter = interpreter_mut.as_mut().unwrap();
            let mut context = interpreter.context.take();
            if let Some(code) = context.as_mut().unwrap().detach_code() {
                interpreter.code.push((0, code));
            }
            loop {
                if let Ok(_) = lock.try_lock() {
                    match interpreter.run(&mut context.as_mut().unwrap()) {
                        Ok(true) => {}
                        Ok(false) => {
                            *interpreter_mut = None;
                            continue 'control;
                        }
                        Err(err) => {
                            println!("Error: {:?}", err);
                            *interpreter_mut = None;
                            continue 'control;
                        }
                    }
                } else {
                    interpreter.context = context;
                    continue 'control;
                }
            }
        }
    }
    
    pub fn run(&mut self, context: &mut ContextData) -> Result<bool, Fault> {
        let mut index = self.code.last().expect("Expected last frame").0;
        let index_copy = index;
        let bytecode = self.code.last().expect("Expected last frame").1.clone();
        if index >= bytecode.len() {
            return Ok(false);
        }
        let result = self.interpret(&mut index, context, &bytecode[index_copy])?;
        if index_copy == index {
            index += 1;
        }
        self.code.last_mut().expect("Expected last frame").0 = index;


        Ok(result)
    }
    

    fn interpret(&mut self, index: &mut usize, context: &mut ContextData, bytecode: &ByteCode) -> Result<bool, Fault> {
        match bytecode {
            ByteCode::Halt => return Ok(false),
            ByteCode::NoOp => {}
            ByteCode::AccessField(index) => self.access_field(context, *index),
            ByteCode::AccessTemp(index) => self.access_temp(*index, context),
            ByteCode::PushLiteral(literal) => self.push_literal(context, literal),
            ByteCode::StoreField(index) => self.store_field(context, *index),
            ByteCode::StoreTemp(index) => self.store_temp(*index, context),
            ByteCode::SendMsg(arg, msg_index) => self.send_msg(*arg, msg_index, context)?,
            ByteCode::SendSuperMsg(arg, msg_index) => self.send_super_msg(*arg, msg_index, context)?,
            ByteCode::SpecialInstruction(instruction) => return self.special_instruction(index, context, instruction),
            _ => unimplemented!()
        }
        Ok(true)
    }

    fn access_field(&self, context: &mut ContextData, index: usize) {
        let value = context.top().expect("Expected value");
        let value = value.borrow();
        let value = value.get_field(index).expect("Expected field").clone();
        context.push(value);
    }

    fn access_temp(&self, index: usize, context: &mut ContextData) {
        
        let value = context.arguments[index].clone();
        context.push(value);
    }

    fn push_literal(&self, context: &mut ContextData, literal: &Literal) {
        let object = match literal {
            Literal::String(string) => crate::object::create_string(string.to_string()),
            Literal::I8(i) => crate::object::create_i8(*i),
            Literal::I16(i) => crate::object::create_i16(*i),
            Literal::I32(i) => crate::object::create_i32(*i),
            Literal::I64(i) => crate::object::create_i64(*i),
            Literal::U8(i) => crate::object::create_u8(*i),
            Literal::U16(i) => crate::object::create_u16(*i),
            Literal::U32(i) => crate::object::create_u32(*i),
            Literal::U64(i) => crate::object::create_u64(*i),
            Literal::F32(f) => crate::object::create_f32(*f),
            Literal::F64(f) => crate::object::create_f64(*f),
            Literal::Boolean(b) => crate::object::create_boolean(*b),
            Literal::Nil => Nil::new(),
            Literal::ByteCode(bytecode) => crate::object::create_block(bytecode.to_vec()),
        };
        context.push(object);
    }

    fn store_field(&self, context: &mut ContextData, index: usize) {
        let value = context.pop().expect("Expected value");
        let object = context.top().expect("Expected object");

        let mut object = object.borrow_mut();
        object.set_field(index, value);
    }

    fn store_temp(&self, index: usize, context: &mut ContextData) {
        let value = context.pop().expect("Expected value");
        context.set_argument(index, value);
    }

    fn send_msg(&mut self, arg: usize, msg_index: &str, context: &mut ContextData) -> Result<(), Fault>{
        for i in 0..arg {
            let value = context.pop().expect("Expected argument");
            context.set_argument(i, value)
        }
        context.arg_count = arg;
        let object = context.top().expect("Stack was empty").clone();
        let borrowed_object = object.borrow();

        let message = crate::object::create_message(&msg_index);

        let method = borrowed_object.process_message(message);
        drop(borrowed_object);
        if let Some(method) = method {
            match *method {
                Method::RustMethod { ref fun } => {
                    match fun(object.clone(), context) {
                        Ok(Some(result)) => context.push(result),
                        Ok(None) => {}
                        Err(err) => return Err(err)
                    }
                }
                Method::BytecodeMethod { ref block } => {
                    context.push_frame(None);
                    context.attach_receiver(object.clone());
                    let object = block.borrow();
                    let object = object.downcast_ref::<Block>().expect("Expected block");
                    let bytecode = object.bytecode.clone();
                    self.code.push((0, bytecode));
                    //context.pop_frame();

                }
            }
        } else {
            return Err(Fault::MethodNotFound(msg_index.to_string()));
        }
        Ok(())
    }

    fn send_super_msg(&mut self, arg: usize, msg_index: &str, context: &mut ContextData) -> Result<(), Fault> {
        for i in 0..arg {
            let value = context.pop().expect("Expected argument");
            context.set_argument(i, value)
        }
        context.arg_count = arg;
        let object = context.top().expect("Stack was empty").clone();
        let borrowed_object = object.borrow();

        let message = crate::object::create_message(&msg_index);
        
        let parent = borrowed_object.get_super_object().expect("Expected super object");
        let borrowed_parent = parent.borrow();

        let method = borrowed_parent.process_message(message);
        drop(borrowed_parent);
        if let Some(method) = method {
            match *method {
                Method::RustMethod { ref fun } => {
                    match fun(parent.clone(), context) {
                        Ok(Some(result)) => context.push(result),
                        Ok(None) => {}
                        Err(err) => return Err(err)
                    }
                }
                Method::BytecodeMethod { ref block } => {
                    context.push_frame(None);
                    context.attach_receiver(parent);
                    let object = block.borrow();
                    let object = object.downcast_ref::<Block>().expect("Expected block");
                    let bytecode = object.bytecode.clone();
                    self.code.push((0, bytecode));
                    //context.pop_frame();
                }
            }
            if let Some(code) = context.detach_code() {
                self.code.push((0, code));
            }
        } else {
            return Err(Fault::MethodNotFound(msg_index.to_string()));
        }
        Ok(())
    }
    
    fn special_instruction(&self, index: &mut usize, context: &mut ContextData, instruction: &SpecialInstruction) -> Result<bool, Fault> {
        match instruction {
            SpecialInstruction::DupStack => Self::dup_stack(context),
            SpecialInstruction::DiscardStack => Self::discard_stack(context),
            SpecialInstruction::ReturnStack => Self::return_stack(context),
            SpecialInstruction::Return => Self::return_(context),
            SpecialInstruction::PopTrueSkip(skip) => Self::pop_true_skip(context, index, *skip),
            SpecialInstruction::PopFalseSkip(skip) => Self::pop_false_skip(context, index, *skip),
            SpecialInstruction::PopTrueBackSkip(skip) => Self::pop_true_back_skip(context, index, *skip),
            SpecialInstruction::PopFalseBackSkip(skip) => Self::pop_false_back_skip(context, index, *skip),
            SpecialInstruction::Skip(skip) => {
                *index += skip;
                Ok(true)
            },
            SpecialInstruction::BackSkip(skip) => {
                *index -= skip;
                Ok(true)
            }
        }
    }
    
    fn dup_stack(context: &mut ContextData) -> Result<bool, Fault> {
        let value = context.top().expect("Expected value").clone();
        let value_ref = value.borrow();
        let value = value_ref.duplicate();

        context.push(value);
        Ok(true)
    }

    fn discard_stack(context: &mut ContextData) -> Result<bool, Fault> {
        context.pop();
        Ok(true)
    }

    fn return_stack(context: &mut ContextData) -> Result<bool, Fault> {
        let value = context.pop().expect("Expected value").clone();
        let frame = context.pop_frame();
        context.push(value);
        context.push_frame(frame);
        Ok(false)
    }

    fn return_(context: &mut ContextData) -> Result<bool, Fault> {
        //let value = context.pop().expect("Expected value").clone();
        let _ = context.pop_frame();
        Ok(false)
    }
    
    fn pop_true_skip(context: &mut ContextData, index: &mut usize, skip: usize) -> Result<bool, Fault> {
        let value = context.pop().expect("Expected value").clone();
        let value = value.borrow();
        let value = value.downcast_ref::<crate::object::primitive::PrimitiveObject<bool>>().ok_or(Fault::InvalidType(String::from("Expected boolean")))?;
        if value.data {
            *index += skip;
        }
        Ok(true)
    }

    fn pop_false_skip(context: &mut ContextData, index: &mut usize, skip: usize) -> Result<bool, Fault> {
        let value = context.pop().expect("Expected value").clone();
        let value = value.borrow();
        let value = value.downcast_ref::<crate::object::primitive::PrimitiveObject<bool>>().ok_or(Fault::InvalidType(String::from("Expected boolean")))?;
        if !value.data {
            *index += skip;
        }
        Ok(true)
    }

    fn pop_true_back_skip(context: &mut ContextData, index: &mut usize, skip: usize) -> Result<bool, Fault> {
        let value = context.pop().expect("Expected value").clone();
        let value = value.borrow();
        let value = value.downcast_ref::<crate::object::primitive::PrimitiveObject<bool>>().ok_or(Fault::InvalidType(String::from("Expected boolean")))?;
        if value.data {
            *index -= skip;
        }
        Ok(true)
    }

    fn pop_false_back_skip(context: &mut ContextData, index: &mut usize, skip: usize) -> Result<bool, Fault> {
        let value = context.pop().expect("Expected value").clone();
        let value = value.borrow();
        let value = value.downcast_ref::<crate::object::primitive::PrimitiveObject<bool>>().ok_or(Fault::InvalidType(String::from("Expected boolean")))?;
        if !value.data {
            *index -= skip;
        }
        Ok(true)
    }
}
