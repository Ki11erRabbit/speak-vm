use crate::object::{ContextData, Method, Nil, Object, ObjectBox};
use crate::object::block::Block;
use crate::vm::bytecode::{ByteCode, SpecialInstruction};

use super::bytecode::Literal;


pub struct Interpreter {
}

impl Interpreter {

    pub fn run(context: &mut ContextData, bytecode: ByteCode) -> bool {
        match bytecode {
            ByteCode::Halt => return false,
            ByteCode::NoOp => {}
            ByteCode::AccessField(index) => Self::access_field(context, index),
            ByteCode::AccessTemp(index) => Self::access_temp(index, context),
            ByteCode::PushLiteral(literal) => Self::push_literal(context, literal),
            ByteCode::StoreField(index) => Self::store_field(context, index),
            ByteCode::StoreTemp(index) => Self::store_temp(index, context),
            ByteCode::SendMsg(arg, msg_index) => Self::send_msg(arg, msg_index, context),
            ByteCode::SendSuperMsg(arg, msg_index) => Self::send_super_msg(arg, msg_index, context),
            ByteCode::SpecialInstruction(instruction) => return Self::special_instruction(context, instruction),
            _ => unimplemented!()
        }
        true
    }

    fn access_field(context: &mut ContextData, index: usize) {
        let value = context.top().expect("Expected value");
        let value = value.borrow();
        let value = value.get_field(index).expect("Expected field").clone();
        context.push(value);
    }

    fn access_temp(index: usize, context: &mut ContextData) {
        
        let value = context.arguments[index].clone();
        context.push(value);
    }

    fn push_literal(context: &mut ContextData, literal: Literal) {
        let object = match literal {
            Literal::String(string) => crate::object::create_string(string),
            Literal::I8(i) => crate::object::create_i8(i),
            Literal::I16(i) => crate::object::create_i16(i),
            Literal::I32(i) => crate::object::create_i32(i),
            Literal::I64(i) => crate::object::create_i64(i),
            Literal::U8(i) => crate::object::create_u8(i),
            Literal::U16(i) => crate::object::create_u16(i),
            Literal::U32(i) => crate::object::create_u32(i),
            Literal::U64(i) => crate::object::create_u64(i),
            Literal::F32(f) => crate::object::create_f32(f),
            Literal::F64(f) => crate::object::create_f64(f),
            Literal::Boolean(b) => crate::object::create_boolean(b),
            Literal::Nil => Nil::new(),
            Literal::ByteCode(bytecode) => crate::object::create_block(bytecode),
        };
        context.push(object);
    }

    fn store_field(context: &mut ContextData, index: usize) {
        let value = context.pop().expect("Expected value");
        let object = context.top().expect("Expected object");

        let mut object = object.borrow_mut();
        object.set_field(index, value);
    }

    fn store_temp(index: usize, context: &mut ContextData) {
        let value = context.pop().expect("Expected value");
        context.set_argument(index, value);
    }

    fn send_msg(arg: usize, msg_index: String, context: &mut ContextData) {
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
                        Err(_) => unimplemented!("Implement errors")
                    }
                }
                Method::BytecodeMethod { ref block } => {
                    context.push_frame(None);
                    context.attach_receiver(object.clone());
                    let object = block.borrow();
                    let object = object.downcast_ref::<Block>().expect("Expected block");
                    for code in object.bytecode.iter() {
                        Self::run(context, code.clone());
                    }
                    context.pop_frame();

                }
            }
        } else {
            unimplemented!("No error handling for missing methods yet.")
        }
    }

    fn send_super_msg(arg: usize, msg_index: String, context: &mut ContextData) {
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
                        Err(_) => unimplemented!("Implement errors")
                    }
                }
                Method::BytecodeMethod { ref block } => {
                    context.push_frame(None);
                    context.attach_receiver(parent);
                    let object = block.borrow();
                    let object = object.downcast_ref::<Block>().expect("Expected block");
                    for code in object.bytecode.iter() {
                        Self::run(context, code.clone());
                    }
                    context.pop_frame();
                }
            }
        } else {
            unimplemented!("No error handling for missing methods yet.")
        }
    }
    
    fn special_instruction(context: &mut ContextData, instruction: SpecialInstruction) -> bool {
        match instruction {
            SpecialInstruction::DupStack => Self::dup_stack(context),
            SpecialInstruction::DiscardStack => Self::discard_stack(context),
            SpecialInstruction::ReturnStack => Self::return_stack(context),
        }
    }
    
    fn dup_stack(context: &mut ContextData) -> bool {
        let value = context.top().expect("Expected value").clone();
         
        let object = crate::object::object_clone(value);

        context.push(object);
        true
    }

    fn discard_stack(context: &mut ContextData) -> bool {
        context.pop();
        true
    }

    fn return_stack(context: &mut ContextData) -> bool {
        let value = context.pop().expect("Expected value").clone();
        let frame = context.pop_frame();
        context.push(value);
        context.push_frame(frame);
        false
    }

}
