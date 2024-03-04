pub mod object;
pub mod vm;

use crate::object::Context;
use crate::object::bytecode::ByteCode;
use crate::object::stack::Stack;
use crate::object::primitive::integer::I64Object;
use crate::object::interpreter::Interpreter;

fn main() {
    let mut context = Context::new();

    let x = I64Object::make_object(context.get_class("I64").unwrap(), context.create_base_object(), 8);
    let y = I64Object::make_object(context.get_class("I64").unwrap(), context.create_base_object(), 8);

    let instructions = vec![
        ByteCode::PushLiteral(x),
        ByteCode::PushLiteral(y),
        ByteCode::SendMsg(1,String::from("add")),
        ByteCode::Halt
    ];
    
    let stack_frame = Stack::make_object(context.get_class("Stack").unwrap(), context.create_base_object());

    let runtime_stack = Stack::make_object_with_stack(context.get_class("Stack").unwrap(), context.create_base_object(), vec![stack_frame]);

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
