use super::{create_message, create_stack, Context, ContextData, Message};
use super::{Object, ObjectBox};
use super::bytecode::{ByteCode, SpecialInstruction};
use super::stack::Stack;
use super::Method;
use super::block::Block;



pub struct Interpreter {
    pub stack: ObjectBox<dyn Object>,
}

impl Interpreter {
    pub fn new(stack: ObjectBox<dyn Object>) -> Self {
        Self { stack }
    }

    pub fn run(&mut self, context: &mut ContextData, bytecode: ByteCode) -> bool {
        match bytecode {
            ByteCode::Halt => return false,
            ByteCode::NoOp => {}
            ByteCode::AccessField(index) => self.access_field(index),
            ByteCode::AccessTemp(index) => self.access_temp(index, context),
            ByteCode::PushLiteral(literal) => self.push_literal(literal),
            ByteCode::StoreField(index) => self.store_field(index),
            ByteCode::StoreTemp(index) => self.store_temp(index, context),
            ByteCode::SendMsg(arg, msg_index) => self.send_msg(arg, msg_index, context),
            ByteCode::SendSuperMsg(arg, msg_index) => self.send_super_msg(arg, msg_index, context),
            ByteCode::SpecialInstruction(instruction) => return self.special_instruction(instruction),
            _ => unimplemented!()
        }
        true
    }

    fn access_field(&mut self, index: usize) {
        let stack = self.stack.borrow();
        let stack = stack.downcast_ref::<Stack>().expect("Expected stack");
        let stack_frame = stack.data.last().expect("Expected stack frame").clone();
        let mut stack_frame = stack_frame.borrow_mut();
        let stack_frame = stack_frame.downcast_mut::<Stack>().expect("Expected stack frame");
        let value = stack_frame.index(0).expect("Expected value").clone();
        let value = value.borrow();
        let value = value.get_field(index).expect("Expected field").clone();
        stack_frame.push(value);
    }

    fn access_temp(&mut self, index: usize, context: &mut ContextData) {
        let stack = self.stack.borrow();
        let stack = stack.downcast_ref::<Stack>().expect("Expected stack");
        let stack_frame = stack.data.last().expect("Expected stack frame").clone();
        let mut stack_frame = stack_frame.borrow_mut();
        let stack_frame = stack_frame.downcast_mut::<Stack>().expect("Expected stack frame");
        
        let value = context.arguments[index].clone();
        stack_frame.push(value);
    }

    fn push_literal(&mut self, literal: ObjectBox<dyn Object>) {
        let stack = self.stack.borrow();
        let stack = stack.downcast_ref::<Stack>().expect("Expected stack");
        let stack_frame = stack.data.last().expect("Expected stack frame").clone();
        let mut stack_frame = stack_frame.borrow_mut();
        let stack_frame = stack_frame.downcast_mut::<Stack>().expect("Expected stack frame");
        stack_frame.push(literal);
    }

    fn store_field(&mut self, index: usize) {
        let stack = self.stack.borrow();
        let stack = stack.downcast_ref::<Stack>().expect("Expected stack");
        let stack_frame = stack.data.last().expect("Expected stack frame").clone();
        let mut stack_frame = stack_frame.borrow_mut();
        let stack_frame = stack_frame.downcast_mut::<Stack>().expect("Expected stack frame");
        let value = stack_frame.pop().expect("Expected value");
        let object = stack_frame.data.last().expect("Expected object");

        let mut object = object.borrow_mut();
        object.set_field(index, value);
    }

    fn store_temp(&mut self, index: usize, context: &mut ContextData) {
        let stack = self.stack.borrow();
        let stack = stack.downcast_ref::<Stack>().expect("Expected stack");
        let stack_frame = stack.data.last().expect("Expected stack frame").clone();
        let mut stack_frame = stack_frame.borrow_mut();
        let stack_frame = stack_frame.downcast_mut::<Stack>().expect("Expected stack frame");
        let value = stack_frame.pop().expect("Expected value");
        context.arguments[index] = value;
    }

    fn send_msg(&mut self, arg: usize, msg_index: String, context: &mut ContextData) {
        let stack = self.stack.clone();
        let mut stack = stack.borrow_mut();
        let stack = stack.downcast_mut::<Stack>().expect("Expected Stack");
        let stack_frame = stack.data.last().expect("Expected stack frame").clone();
        let mut stack_frame = stack_frame.borrow_mut();
        let stack_frame = stack_frame.downcast_mut::<Stack>().expect("Expected stack frame");
        let mut args = Vec::new();
        for _ in 0..arg {
            args.push(stack_frame.pop().expect("Expected argument"));
        }
        context.arguments = args;
        let object = stack_frame.data.last().expect("Stack was empty").clone();
        let borrowed_object = object.borrow();

        let message = create_message(&msg_index);

        let method = borrowed_object.process_message(message);
        drop(borrowed_object);
        if let Some(method) = method {
            match *method {
                Method::RustMethod { ref fun } => {
                    let new_frame = create_stack();
                    stack.push(new_frame);
                    match fun(object.clone(), context, self) {
                        Ok(Some(result)) => stack_frame.push(result),
                        Ok(None) => {}
                        Err(_) => unimplemented!("Implement errors")
                    }
                    stack.pop();
                }
                Method::BytecodeMethod { ref block } => {
                    let stack_frame = create_stack();
                    stack.push(stack_frame);
                    context.attach_receiver(object.clone());
                    let object = block.borrow();
                    let object = object.downcast_ref::<Block>().expect("Expected block");
                    for code in object.bytecode.iter() {
                        self.run(context, code.clone());
                    }
                    stack.pop();

                }
            }
        }
    }

    fn send_super_msg(&mut self, arg: usize, msg_index: String, context: &mut ContextData) {
        let stack = self.stack.clone();
        let mut stack = stack.borrow_mut();
        let stack = stack.downcast_mut::<Stack>().expect("Expected Stack");
        let stack_frame = stack.data.last().expect("Expected stack frame").clone();
        let mut stack_frame = stack_frame.borrow_mut();
        let stack_frame = stack_frame.downcast_mut::<Stack>().expect("Expected stack frame");
        let mut args = Vec::new();
        for _ in 0..arg {
            args.push(stack_frame.pop().expect("Expected argument"));
        }
        context.arguments = args;
        let object = stack_frame.data.last().expect("Stack was empty").clone();
        let borrowed_object = object.borrow();

        let message = create_message(&msg_index);
        
        let parent = borrowed_object.get_super_object().expect("Expected super object");
        let borrowed_parent = parent.borrow();

        let method = borrowed_parent.process_message(message);
        drop(borrowed_parent);
        if let Some(method) = method {
            match *method {
                Method::RustMethod { ref fun } => {
                    let new_frame = create_stack();
                    stack.push(new_frame);
                    match fun(parent.clone(), context, self) {
                        Ok(Some(result)) => stack_frame.push(result),
                        Ok(None) => {}
                        Err(_) => unimplemented!("Implement errors")
                    }
                    stack.pop();
                }
                Method::BytecodeMethod { ref block } => {
                    let stack_frame = create_stack();
                    stack.push(stack_frame);
                    context.attach_receiver(parent);
                    let object = block.borrow();
                    let object = object.downcast_ref::<Block>().expect("Expected block");
                    for code in object.bytecode.iter() {
                        self.run(context, code.clone());
                    }
                    stack.pop();

                }
            }
        }
    }
    
    fn special_instruction(&mut self, instruction: SpecialInstruction) -> bool {
        match instruction {
            SpecialInstruction::DupStack => self.dup_stack(),
            SpecialInstruction::DiscardStack => self.discard_stack(),
            SpecialInstruction::ReturnStack => self.return_stack(),
        }
    }
    
    fn dup_stack(&mut self) -> bool {
        let stack = self.stack.borrow();
        let stack = stack.downcast_ref::<Stack>().expect("Expected stack");
        let stack_frame = stack.data.last().expect("Expected stack frame").clone();
        let mut stack_frame = stack_frame.borrow_mut();
        let stack_frame = stack_frame.downcast_mut::<Stack>().expect("Expected stack frame");
        let value = stack_frame.index(0).expect("Expected value").clone();
        
        let object = crate::object::object_clone(value);

        stack_frame.push(object);
        true
    }

    fn discard_stack(&mut self) -> bool {
        let stack = self.stack.borrow();
        let stack = stack.downcast_ref::<Stack>().expect("Expected stack");
        let stack_frame = stack.data.last().expect("Expected stack frame").clone();
        let mut stack_frame = stack_frame.borrow_mut();
        let stack_frame = stack_frame.downcast_mut::<Stack>().expect("Expected stack frame");
        stack_frame.pop();
        true
    }

    fn return_stack(&mut self) -> bool {
        let stack = self.stack.clone();
        let mut stack = stack.borrow_mut();
        let stack = stack.downcast_mut::<Stack>().expect("Expected stack");
        let current_frame = stack.pop().expect("Expected stack frame").clone();
        let mut current_frame = current_frame.borrow_mut();
        let current_frame = current_frame.downcast_mut::<Stack>().expect("Expected stack frame");
        let value = current_frame.pop().expect("Expected value").clone();
        let frame = stack.data.last().expect("Expected stack frame").clone();
        let mut frame = frame.borrow_mut();
        let frame = frame.downcast_mut::<Stack>().expect("Expected stack frame");
        frame.push(value);
        false
    }

}
