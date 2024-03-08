use std::sync::Arc;
use std::collections::HashMap;
use crate::object::ContextData;
use crate::object::string::StringObject;
use crate::object::Method;
use std::io::Write;
use log::{info, warn, error, debug, trace};

use super::{Fault, Object, ObjectBox};
use crate::object::VTable;














pub struct Logger {
    super_object: Option<ObjectBox>,
    vtable: VTable,
}


impl Logger {
    pub fn make_object(parent: ObjectBox) -> ObjectBox {
        let logger = Logger {
            super_object: Some(parent),
            vtable: VTable::new_empty(),
        };
        ObjectBox::new(logger)
    }
    pub fn make_vtable() -> VTable {
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

        VTable::new(methods)
    }
}


impl Object for Logger {
    fn get_vtable(&self) -> &VTable {
        &self.vtable
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        self.super_object.clone()
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox> {
        panic!("Integer objects do not have fields")
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox) {
        panic!("Integer objects do not have fields")
    }
    fn size(&self) -> Option<usize> {
        None
    }
    fn duplicate(&self) -> ObjectBox {
        let logger = Logger::make_object(self.super_object.clone().unwrap().borrow().duplicate());
        let mut log = logger.borrow_mut();
        log.initialize(vec![], self.vtable.clone());
        drop(log);
        logger
    }
    fn initialize(&mut self, _: Vec<ObjectBox>, vtable: VTable) {
        self.vtable.extend(Logger::make_vtable());
        self.vtable.extend(vtable);
        match &mut self.super_object {
            Some(super_object) => {
                let mut super_object = super_object.borrow_mut();
                super_object.initialize(vec![], VTable::new_empty());
            }
            None => {}
        }
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
