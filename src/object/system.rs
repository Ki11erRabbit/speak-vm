use std::sync::Arc;
use std::collections::HashMap;
use crate::object::{block::Block, ContextData};
use crate::object::string::StringObject;
use crate::object::Method;
use crate::SEND_CHANNEL;

use super::stack::Stack;
use super::{Fault, Object, ObjectBox};
use crate::object::VTable;



pub struct System {
    super_object: Option<ObjectBox>,
    vtable: VTable,
}

impl System {
    pub fn make_object(parent: ObjectBox) -> ObjectBox {
        let system = System {
            super_object: Some(parent),
            vtable: VTable::new_empty(),
        };
        ObjectBox::new(system)
    }
    pub fn make_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert("spawn".to_string(), Arc::new(Method::RustMethod { fun: Box::new(system_spawn)}));
        methods.insert("stack".to_string(), Arc::new(Method::RustMethod { fun: Box::new(system_get_stack)}));
        methods.insert("current_frame".to_string(), Arc::new(Method::RustMethod { fun: Box::new(system_get_current_frame)}));
        VTable::new(methods)
    }
}

impl Object for System {
    fn get_vtable(&self) -> &VTable {
        &self.vtable
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        self.super_object.clone()
    }
    fn get_field(&self, _: usize) -> Option<ObjectBox> {
        panic!("System object has no fields")
    }
    fn set_field(&mut self, _: usize, _: ObjectBox) {
        panic!("System object has no fields")
    }
    fn size(&self) -> Option<usize> {
        None
    }
    fn duplicate(&self) -> ObjectBox {
        let object = System::make_object(self.super_object.clone().unwrap().borrow().duplicate());
        let mut object_mut = object.borrow_mut();
        object_mut.initialize(vec![], self.vtable.clone());
        drop(object_mut);
        object
    }
    fn initialize(&mut self, _args: Vec<ObjectBox>, vtable: VTable) {
        self.vtable.extend(System::make_vtable());
        self.vtable.extend(vtable);
        match self.super_object.clone() {
            Some(super_object) => {
                let mut super_object = super_object.borrow_mut();
                super_object.initialize(vec![], self.vtable.clone());
                drop(super_object);
            }
            None => {}
        }
    }
}


fn system_spawn(_: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let block = context.arguments[0].clone();
    let block = block.borrow();
    let block = block.downcast_ref::<Block>().ok_or(Fault::InvalidType(format!("System spawn: argument was not a Block")))?;
    let mut new_context = ContextData::new(super::init_stack());
    for (i, capture) in block.captures.iter().enumerate() {
        new_context.set_argument(i, capture.clone())
    }
    let _ = block.call(&mut new_context);
    SEND_CHANNEL.lock().unwrap().as_mut().unwrap().send(new_context).unwrap();

    Ok(None)
}

fn system_get_stack(_: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let stack = context.stack.clone();
    Ok(Some(stack))
}

fn system_get_current_frame(_: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let stack = context.stack.clone();
    let stack = stack.borrow();
    let stack = stack.downcast_ref::<Stack>();
    let stack = stack.expect("Expected stack");
    let frame = stack.index(0).unwrap();
    Ok(Some(frame))
}
