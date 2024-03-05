use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::object::{Class, Object, ObjectBox};
use crate::object::Fault;
use crate::vm::interpreter::Interpreter;
use crate::object::Method;
use std::sync::Arc;

use crate::vm::bytecode::ByteCode;
use super::{ContextData, VTable};







pub struct Block {
    class: Arc<Class>,
    super_object: ObjectBox,
    pub bytecode: Vec<ByteCode>,
    vtable: VTable,
}


impl Block {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let mut methods = HashMap::new();
        methods.insert(String::from("value"), Arc::new(Method::RustMethod { fun: Box::new(value) }));
        Class::new(Some(parent), methods)
    }
    pub fn make_object(class: Arc<Class>,
                       parent: ObjectBox,
                       bytecode: Vec<ByteCode>) -> ObjectBox {
        Rc::new(RefCell::new(Block {class, super_object: parent, bytecode, vtable: class.get_vtable()})) as ObjectBox
    }
}

impl Object for Block {
    fn get_class(&self) -> Arc<Class> {
        self.class.clone()
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        Some(self.super_object.clone())
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox> {
        panic!("Block objects do not have fields")
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox) {
        panic!("Block objects do not have fields")
    }
    fn size(&self) -> Option<usize> {
        None
    }
    fn duplicate(&self) -> ObjectBox {
        Block::make_object(self.class.clone(), self.super_object.clone(), self.bytecode.clone())
    }
    fn initalize(&mut self, _: Vec<ObjectBox>, vtable: VTable) {
        self.vtable.extend(vtable);
    }
}

fn value(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    let object = object.downcast_ref::<Block>().expect("Expected block");
    for code in object.bytecode.iter() {
        Interpreter::run(context, code.clone());
    }
    Ok(None)
}
