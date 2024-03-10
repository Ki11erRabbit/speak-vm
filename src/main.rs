pub mod object;
pub mod vm;

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, RwLock, TryLockError};

use object::{init_stack, Class, ContextData, Method};
use vm::bytecode::Literal;
use object::ObjectBox;

use crate::vm::bytecode::ByteCode;
use crate::vm::interpreter::Interpreter;
use clap::Parser;


#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    thread_count: Option<usize>,
    #[clap(short, long)]
    server_mode: bool,
    #[clap(short, long)]
    object_files: Vec<String>,
    args: Vec<String>
}

impl Args {
    fn into_iter(self) -> std::vec::IntoIter<String> {
        self.args.into_iter()
    }
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
    for file in &args.object_files {
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
    methods.insert("main".to_string(), Arc::new(Method::BytecodeMethod{ block: object::create_block(bytecode) }));
    let vtable = object::VTable::new(methods);
    let overrides = vec![];
    let class = Class::new(Some("Object"), vtable, overrides);
    object::add_class("Main", class);


    let instructions = vec![
        ByteCode::StoreTemp(3),
        ByteCode::AccessTemp(3),
        ByteCode::PushLiteral(Literal::String(String::from("hello_world"))),
        ByteCode::SendMsg(1,String::from("new")),
        ByteCode::SendMsg(0,String::from("init")),
        ByteCode::SendMsg(0,String::from("println")),
        ByteCode::Halt
    ];
    
    if args.server_mode {
        unimplemented!()

    } else {
        let arguments: Vec<ObjectBox> = args.into_iter().map(|x| object::create_string(x)).collect();
        for (i, arg) in arguments.iter().enumerate() {
            context.set_argument(i, arg.clone());
        }
        let main = object::create_object("Main", &arguments)?.ok_or("No main object found")?;
        let main = main.borrow();
        let message = object::create_message("main");
        let main_method = main.process_message(message);
        let main_method = main_method.ok_or("No main method found")?;
        match &*main_method {
            Method::BytecodeMethod { block } => {
                let block = block.borrow();
                let block = block.downcast_ref::<object::block::Block>().ok_or("Block not found")?;
                let _ = block.call(&mut context)?;
            }
            _ => unimplemented!()
        }
    }
    let mut tasks = VecDeque::new();
    let mut current_tasks: Arc<RwLock<Vec<Arc<Mutex<Interpreter>>>>> = Arc::new(RwLock::new(Vec::new()));
    let mut next_task = 0;
    let lock = Arc::new(Mutex::new(()));

    let (sender, receiver) = std::sync::mpsc::channel();
    sender.send(context.detach_code().expect("no main method")).unwrap();
    let mut start_time = std::time::Instant::now();

    loop {
        match receiver.try_recv() {
            Ok(instructions) => {
                let mut context = ContextData::new(init_stack());
                context.attach_code(instructions);
                let interpreter = Interpreter::new(context);
                tasks.push_back(Arc::new(Mutex::new(interpreter)));
                next_task += 1;
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                break;
            }
        }
        if start_time.elapsed().as_millis() < 100 {
            continue;
        }

        if let Ok(guard) = lock.lock() {
            let mut current_tasks = current_tasks.write().expect("Could not write to current tasks");
            for i in 0..next_task {
                let mut task = tasks.pop_front();
                if current_tasks.len() <= i {
                    if let Some(task) = task {
                        current_tasks.push(task);
                        continue;
                    }
                }
                let current_task = current_tasks[i].clone();
                // Here false is back and true is front
                let mut back_or_front = true;
                match current_task.try_lock() {
                    Ok(_) => {
                        back_or_front = false;
                    }
                    Err(TryLockError::WouldBlock) => {

                    }
                    Err(x) => {
                        eprintln!("{:?}", x);
                        return Ok(())
                    }
                };
                if back_or_front {
                    if let Some(task) = task {
                        tasks.push_front(task);
                    }
                } else {
                    drop(current_task);
                    if let Some(ref mut task) = task {
                        std::mem::swap(task, &mut current_tasks[i]);
                    }
                    if let Some(task) = task {
                        tasks.push_back(task);
                    }
                }
            };

            drop(guard);
        }
        start_time = std::time::Instant::now();
        
    }

    //Interpreter::run_normal(&instructions, &mut context)?;



    Ok(())

}
