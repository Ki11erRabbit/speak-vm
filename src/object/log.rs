use std::sync::Arc;
use std::collections::HashMap;
use crate::object::ContextData;
use crate::object::string::StringObject;
use crate::object::Method;
use std::io::Write;
use log::{info, warn, error, debug, trace};

use super::{Class, Fault, Object, ObjectBox, ObjectStruct};















pub struct Logger {}


impl Logger {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let mut methods = HashMap::new();
        methods.insert("println".to_string(), Arc::new(Method::RustMethod { fun: Box::new(log_println)}));
        methods.insert("print".to_string(), Arc::new(Method::RustMethod { fun: Box::new(log_print)}));
        methods.insert("eprintln".to_string(), Arc::new(Method::RustMethod { fun: Box::new(log_eprintln)}));
        methods.insert("eprint".to_string(), Arc::new(Method::RustMethod { fun: Box::new(log_eprint)}));
        methods.insert("info".to_string(), Arc::new(Method::RustMethod { fun: Box::new(log_info)}));
        methods.insert("trace".to_string(), Arc::new(Method::RustMethod { fun: Box::new(log_trace)}));
        methods.insert("warn".to_string(), Arc::new(Method::RustMethod { fun: Box::new(log_warn)}));
        methods.insert("error".to_string(), Arc::new(Method::RustMethod { fun: Box::new(log_error)}));
        methods.insert("debug".to_string(), Arc::new(Method::RustMethod { fun: Box::new(log_debug)}));

        Class::new(Some(parent), methods)
    }
    pub fn make_object(class: Arc<Class>, parent: ObjectBox) -> ObjectBox {
        ObjectStruct::new(class, Some(parent))
    }
}



fn log_println(_: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let message = context.arguments[0].clone();
    let message = message.borrow();
    let message = message.downcast_ref::<StringObject>().ok_or(Fault::InvalidType)?;
    println!("{}", message.value);
    Ok(None)
}

fn log_print(_: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let message = context.arguments[0].clone();
    let message = message.borrow();
    let message = message.downcast_ref::<StringObject>().ok_or(Fault::InvalidType)?;
    print!("{}", message.value);
    std::io::stdout().flush().map_err(|x| Fault::IO(x))?;
    Ok(None)
}

fn log_eprintln(_: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let message = context.arguments[0].clone();
    let message = message.borrow();
    let message = message.downcast_ref::<StringObject>().ok_or(Fault::InvalidType)?;
    println!("{}", message.value);
    Ok(None)
}

fn log_eprint(_: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let message = context.arguments[0].clone();
    let message = message.borrow();
    let message = message.downcast_ref::<StringObject>().ok_or(Fault::InvalidType)?;
    print!("{}", message.value);
    std::io::stdout().flush().map_err(|x| Fault::IO(x))?;
    Ok(None)
}

fn log_info(_: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let message = context.arguments[0].clone();
    let message = message.borrow();
    let message = message.downcast_ref::<StringObject>().ok_or(Fault::InvalidType)?;
    info!("{}", message.value);
    Ok(None)
}

fn log_trace(_: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let message = context.arguments[0].clone();
    let message = message.borrow();
    let message = message.downcast_ref::<StringObject>().ok_or(Fault::InvalidType)?;
    trace!("{}", message.value);
    Ok(None)
}

fn log_warn(_: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let message = context.arguments[0].clone();
    let message = message.borrow();
    let message = message.downcast_ref::<StringObject>().ok_or(Fault::InvalidType)?;
    warn!("{}", message.value);
    Ok(None)
}

fn log_error(_: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let message = context.arguments[0].clone();
    let message = message.borrow();
    let message = message.downcast_ref::<StringObject>().ok_or(Fault::InvalidType)?;
    error!("{}", message.value);
    Ok(None)
}

fn log_debug(_: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let message = context.arguments[0].clone();
    let message = message.borrow();
    let message = message.downcast_ref::<StringObject>().ok_or(Fault::InvalidType)?;
    debug!("{}", message.value);
    Ok(None)
}
