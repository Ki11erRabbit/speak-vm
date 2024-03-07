pub mod object;
pub mod vm;

use std::collections::HashMap;
use std::sync::Arc;

use object::{init_stack, Class, ContextData, Method};
use vm::bytecode::Literal;

use crate::vm::bytecode::ByteCode;
use crate::vm::interpreter::Interpreter;
use clap::Parser;


#[derive(Parser, Debug)]
struct Args {
    object_files: Vec<String>,
}

fn load_object_file(file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data = std::fs::read(file)?;
    let binary = vm::create_binary(data.as_slice()).map_err(|e| format!("Error loading object file: {:?}", e))?;
    for (name, class) in binary.into_iter() {
        object::add_class(&name, class)
    }
    Ok(())
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let args = Args::parse();
    for file in args.object_files {
        load_object_file(&file)?;
    }
    let mut context = ContextData::new(init_stack());

    //let x = create_i64(8);
    //let y = create_i64(8);

    /*let instructions = vec![
        ByteCode::PushLiteral(x),
        ByteCode::PushLiteral(y),
        ByteCode::SendMsg(1,String::from("add")),
        ByteCode::Halt
    ];*/
    
    let bytecode = vec![
        ByteCode::AccessTemp(3),
        ByteCode::PushLiteral(Literal::String(String::from("Logger"))),
        ByteCode::SendMsg(1,String::from("new")),
        ByteCode::SendMsg(0,String::from("init")),
        ByteCode::PushLiteral(Literal::String(String::from("Hello World"))),
        ByteCode::SendMsg(1,String::from("println")),
    ];
    let mut methods = HashMap::new();
    methods.insert("println".to_string(), Arc::new(Method::BytecodeMethod{ block: object::create_block(bytecode) }));
    let vtable = object::VTable::new(methods);
    let overrides = vec![vtable];
    let vtable = object::VTable::new_empty();
    let class = Class::new(Some("Logger"), vtable, overrides);
    object::add_class("hello_world", class);


    let instructions = vec![
        ByteCode::StoreTemp(3),
        ByteCode::AccessTemp(3),
        ByteCode::PushLiteral(Literal::String(String::from("hello_world"))),
        ByteCode::SendMsg(1,String::from("new")),
        ByteCode::SendMsg(0,String::from("init")),
        ByteCode::SendMsg(0,String::from("println")),
        ByteCode::Halt
    ];
    


    for instruction in instructions {
        match Interpreter::run(&mut context, &instruction)? {
            false => {
                println!("Halted");
            }
            _ => {}
        }
    }



    Ok(())

}
