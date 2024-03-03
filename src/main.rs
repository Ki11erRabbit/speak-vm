pub mod object;
pub mod vm;
use object::primitive::integer::IntegerObject;
use object::primitive::NumberObject;
use object::Message;

use crate::object::Context;
use crate::object::bytecode::{ByteCodeObject, ByteCode};
use crate::object::stack::Stack;
use crate::object::primitive::integer::I64Object;
use crate::object::interpreter::Interpreter;

fn main() {
    let mut context = Context::new();
    let bytecode_class = ByteCodeObject::make_class(context.get_class("Object").unwrap().clone());
    let stack_class = Stack::make_class(context.get_class("Object").unwrap().clone());
    let number_class = NumberObject::make_class(context.get_class("Object").unwrap().clone());
    let integer_class = IntegerObject::make_class(Box::new(number_class.clone()));
    let i64_class = I64Object::make_class(Box::new(integer_class.clone()));
    let message_class = Message::make_class(context.get_class("Object").unwrap().clone());
    context.add_class("Bytecode", bytecode_class.clone());
    context.add_class("Stack", stack_class);
    context.add_class("I64", i64_class);
    context.add_class("Message", message_class);
    context.add_class("Number", number_class);

    let x = I64Object::make_object(*context.get_class("I64").unwrap().clone(), context.create_base_object(), 8);
    let y = I64Object::make_object(*context.get_class("I64").unwrap().clone(), context.create_base_object(), 8);

    let instructions = vec![
        ByteCodeObject::make_object(bytecode_class.clone(),context.create_base_object(), ByteCode::PushLiteral(x)),
        ByteCodeObject::make_object(bytecode_class.clone(),context.create_base_object(), ByteCode::PushLiteral(y)),
        ByteCodeObject::make_object(bytecode_class.clone(),context.create_base_object(), ByteCode::SendMsg(1,0)),
        ByteCodeObject::make_object(bytecode_class.clone(),context.create_base_object(), ByteCode::Halt)
    ];
    
    let stack_frame = Stack::make_object(*context.get_class("Stack").unwrap().clone(), context.create_base_object());

    let runtime_stack = Stack::make_object_with_stack(*context.get_class("Stack").unwrap().clone(), context.create_base_object(), vec![stack_frame]);

    let mut interpreter = Interpreter::new(runtime_stack);

    for instruction in instructions {
        match interpreter.run(&mut context, instruction.clone()) {
            false => {
                println!("Halted");
            }
            _ => {}
        }
    }





}
