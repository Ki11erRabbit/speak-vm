use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::object::{Class, Object, ObjectBox};
use crate::object::Context;
use crate::object::Fault;
use crate::object::Interpreter;
use crate::object::Method;
use std::sync::Arc;

use super::bytecode::ByteCode;







pub struct Block {
    class: Arc<Class>,
    super_object: ObjectBox<dyn Object>,
    pub bytecode: Vec<ByteCode>,
}


impl Block {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let mut methods = HashMap::new();
        methods.insert(String::from("value"), Arc::new(Method::RustMethod { fun: Box::new(value) }));
        Class::new(Some(parent), methods)
    }
    pub fn make_object(class: Arc<Class>,
                       parent: ObjectBox<dyn Object>,
                       bytecode: Vec<ByteCode>) -> ObjectBox<dyn Object> {
        Rc::new(RefCell::new(Block {class, super_object: parent, bytecode})) as ObjectBox<dyn Object>
    }
}

impl Object for Block {
    fn get_class(&self) -> Arc<Class> {
        self.class.clone()
    }
    fn get_super_object(&self) -> Option<ObjectBox<dyn Object>> {
        Some(self.super_object.clone())
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox<dyn Object>> {
        panic!("Block objects do not have fields")
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox<dyn Object>) {
        panic!("Block objects do not have fields")
    }
    fn size(&self) -> Option<usize> {
        None
    }
    fn duplicate(&self) -> ObjectBox<dyn Object> {
        Block::make_object(self.class.clone(), self.super_object.clone(), self.bytecode.clone())
    }
}

fn value(object: ObjectBox<dyn Object>, context: &mut Context, interpreter: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    let object = object.borrow();
    let object = object.downcast_ref::<Block>().expect("Expected block");
    for code in object.bytecode.iter() {
        interpreter.run(context, code.clone());
    }
    Ok(None)
}
