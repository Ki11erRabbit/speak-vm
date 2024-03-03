use std::rc::Rc;
use std::cell::RefCell;
use crate::object::{Class, Object, ObjectBox};
use crate::object::Context;
use crate::object::Fault;
use crate::object::Interpreter;
use crate::object::Method;
use std::sync::Arc;







pub struct Block {
    class: Class,
    super_object: ObjectBox<dyn Object>,
    pub bytecode: Vec<ObjectBox<dyn Object>>,
}


impl Block {
    pub fn make_class(parent: Box<Class>) -> Class {
        let mut methods = Vec::new();
        methods.push(Arc::new(Method::RustMethod { fun: Box::new(value) }));
        Class::new(Some(parent), methods)
    }
    pub fn make_object(class: Class,
                       parent: ObjectBox<dyn Object>,
                       bytecode: Vec<ObjectBox<dyn Object>>) -> ObjectBox<dyn Object> {
        Rc::new(RefCell::new(Block {class, super_object: parent, bytecode})) as ObjectBox<dyn Object>
    }
}

impl Object for Block {
    fn get_class(&self) -> &Class {
        &self.class
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
}

fn value(object: ObjectBox<dyn Object>, context: &mut Context, interpreter: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    let object = object.borrow();
    let object = object.downcast_ref::<Block>().expect("Expected block");
    for code in object.bytecode.iter() {
        interpreter.run(context, code.clone());
    }
    Ok(None)
}
