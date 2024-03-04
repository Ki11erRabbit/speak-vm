pub mod object;
pub mod vm;

use object::{create_i64, create_string, init_stack, ContextData};

use crate::object::bytecode::ByteCode;
use crate::object::interpreter::Interpreter;

fn main() {
    let mut context = ContextData::new(init_stack());

    let x = create_i64(8);
    let y = create_i64(8);

    /*let instructions = vec![
        ByteCode::PushLiteral(x),
        ByteCode::PushLiteral(y),
        ByteCode::SendMsg(1,String::from("add")),
        ByteCode::Halt
    ];*/
    let z = create_string(String::from("Hello World"));
    let a = create_string(String::from("Logger"));
    let instructions = vec![
        ByteCode::PushLiteral(a),
        ByteCode::SendMsg(1,String::from("new")),
        ByteCode::PushLiteral(z),
        ByteCode::SendMsg(1,String::from("println")),
        ByteCode::Halt
    ];
    

    let runtime_stack = init_stack();

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
