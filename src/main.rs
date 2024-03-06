pub mod object;
pub mod vm;

use std::collections::HashMap;
use std::sync::Arc;

use object::{create_i64, create_string, init_stack, Class, ContextData, Method};

use crate::vm::bytecode::ByteCode;
use crate::vm::interpreter::Interpreter;

fn main() {
    let mut context = ContextData::new(init_stack());

    //let x = create_i64(8);
    //let y = create_i64(8);

    /*let instructions = vec![
        ByteCode::PushLiteral(x),
        ByteCode::PushLiteral(y),
        ByteCode::SendMsg(1,String::from("add")),
        ByteCode::Halt
    ];*/
    let z = create_string(String::from("Hello World"));
    let a = create_string(String::from("Logger"));
    
    let bytecode = vec![
        ByteCode::AccessTemp(3),
        ByteCode::PushLiteral(a),
        ByteCode::SendMsg(1,String::from("new")),
        ByteCode::SendMsg(0,String::from("init")),
        ByteCode::PushLiteral(z),
        ByteCode::SendMsg(1,String::from("println")),
    ];
    let mut methods = HashMap::new();
    methods.insert("println".to_string(), Arc::new(Method::BytecodeMethod{ block: object::create_block(bytecode) }));
    let vtable = object::VTable::new(methods);
    let overrides = vec![vtable];
    let vtable = object::VTable::new_empty();
    let class = Class::new(Some("Logger"), vtable, overrides);
    object::add_class("hello_world", class);

    let a = create_string(String::from("hello_world"));

    let instructions = vec![
        ByteCode::StoreTemp(3),
        ByteCode::AccessTemp(3),
        ByteCode::PushLiteral(a),
        ByteCode::SendMsg(1,String::from("new")),
        ByteCode::SendMsg(0,String::from("init")),
        ByteCode::SendMsg(0,String::from("println")),
        ByteCode::Halt
    ];
    


    for instruction in instructions {
        match Interpreter::run(&mut context, instruction.clone()) {
            false => {
                println!("Halted");
            }
            _ => {}
        }
    }





}
