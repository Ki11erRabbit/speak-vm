use super::{Context, Message};
use super::{Class, Object, ObjectBox};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use super::bytecode::ByteCode;
use super::bytecode::ByteCodeObject;
use super::stack::Stack;
use std::sync::Arc;
use super::Method;



pub struct Interpreter {
    pub stack: ObjectBox<dyn Object>,
}

impl Interpreter {
    pub fn new(stack: ObjectBox<dyn Object>) -> Self {
        Self { stack }
    }

    pub fn run(&mut self, context: &mut Context, bytecode: ObjectBox<dyn Object>) -> bool {
        let bytecode = bytecode.borrow();
        let bytecode = bytecode.downcast_ref::<ByteCodeObject>();
        if let Some(bytecode) = bytecode {
            match &bytecode.data {
                ByteCode::Halt => return false,
                ByteCode::NoOp => {}
                ByteCode::AccessField(index) => self.access_field(*index),
                ByteCode::AccessTemp(index) => self.access_temp(*index, context),
                ByteCode::PushLiteral(literal) => self.push_literal(literal.clone()),
                ByteCode::StoreField(index) => self.store_field(*index),
                ByteCode::StoreTemp(index) => self.store_temp(*index, context),
                ByteCode::SendMsg(arg, msg_index) => self.send_msg(*arg, *msg_index, context),
                ByteCode::SendSuperMsg(arg, msg_index) => self.send_super_msg(*arg, *msg_index, context),
                _ => unimplemented!()
            }
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

    fn access_temp(&mut self, index: usize, context: &mut Context) {
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

    fn store_temp(&mut self, index: usize, context: &mut Context) {
        let stack = self.stack.borrow();
        let stack = stack.downcast_ref::<Stack>().expect("Expected stack");
        let stack_frame = stack.data.last().expect("Expected stack frame").clone();
        let mut stack_frame = stack_frame.borrow_mut();
        let stack_frame = stack_frame.downcast_mut::<Stack>().expect("Expected stack frame");
        let value = stack_frame.pop().expect("Expected value");
        context.arguments[index] = value;
    }

    fn send_msg(&mut self, arg: usize, msg_index: usize, context: &mut Context) {
        let stack = self.stack.borrow();
        let stack = stack.downcast_ref::<Stack>().expect("Expected Stack");
        let stack_frame = stack.data.last().expect("Expected stack frame").clone();
        let mut stack_frame = stack_frame.borrow_mut();
        let stack_frame = stack_frame.downcast_mut::<Stack>().expect("Expected stack frame");
        let mut args = Vec::new();
        for _ in 0..arg {
            args.push(stack_frame.pop().expect("Expected argument"));
        }
        context.arguments = args;
        let object = stack_frame.data.pop().expect("Stack was empty").clone();
        let borrowed_object = object.borrow();

        let message_class = context.get_class("Message").expect("Expected Message class").clone();
        let message = Message::make_object(*message_class, context.create_base_object(), msg_index);

        let method = borrowed_object.process_message(message);
        drop(borrowed_object);
        if let Some(method) = method {
            match *method {
                Method::RustMethod { ref fun } => {
                    match fun(object.clone(), context) {
                        Ok(Some(result)) => stack_frame.push(result),
                        Ok(None) => {}
                        Err(_) => unimplemented!("Implement errors")
                    }
                }
                Method::BytecodeMethod { .. } => {
                    unimplemented!()
                }
            }
        }
    }

    fn send_super_msg(&mut self, arg: usize, msg_index: usize, context: &mut Context) {
        let stack = self.stack.borrow();
        let stack = stack.downcast_ref::<Stack>().expect("Expected Stack");
        let stack_frame = stack.data.last().expect("Expected stack frame").clone();
        let mut stack_frame = stack_frame.borrow_mut();
        let stack_frame = stack_frame.downcast_mut::<Stack>().expect("Expected stack frame");
        let mut args = Vec::new();
        for _ in 0..arg {
            args.push(stack_frame.pop().expect("Expected argument"));
        }
        context.arguments = args;
        let object = stack_frame.data.pop().expect("Stack was empty").clone();
        let borrowed_object = object.borrow();

        let message_class = context.get_class("Message").expect("Expected Message class").clone();
        let message = Message::make_object(*message_class, context.create_base_object(), msg_index);
        
        let parent = borrowed_object.get_super_object().expect("Expected super object");
        let borrowed_parent = parent.borrow();

        //TODO: Implement override
        let method = borrowed_parent.process_message(message);
        drop(borrowed_parent);
        if let Some(method) = method {
            match *method {
                Method::RustMethod { ref fun } => {
                    match fun(parent.clone(), context) {
                        Ok(Some(result)) => stack_frame.push(result),
                        Ok(None) => {}
                        Err(_) => unimplemented!("Implement errors")
                    }
                }
                Method::BytecodeMethod { .. } => {
                    unimplemented!()
                }
            }
        }
    }
}
