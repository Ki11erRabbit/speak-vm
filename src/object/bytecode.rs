use super::{Class, Object, ObjectBox};
use std::cell::RefCell;
use std::rc::Rc;

pub struct ByteCodeObject {
    class: Class,
    super_object: Option<ObjectBox<dyn Object>>,
    pub data: ByteCode,
}


impl ByteCodeObject {
    pub fn make_class<'a>(parent: Box<Class>) -> Class {
        Class::new(Some(parent), Vec::new())
    }
    pub fn make_object<'a>(class: Class,
                           parent: ObjectBox<dyn Object>,
                           data: ByteCode) -> ObjectBox<dyn Object + 'a> {
        Rc::new(RefCell::new(ByteCodeObject {class, super_object: Some(parent), data})) as ObjectBox<dyn Object + 'a>
    }
}


impl Object for ByteCodeObject {
    fn get_class(&self) -> &Class {
        &self.class
    }
    fn get_super_object(&self) -> Option<ObjectBox<dyn Object>> {
        self.super_object.clone()
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox<dyn Object>> {
        panic!("ByteCode objects do not have fields")
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox<dyn Object>) {
        panic!("ByteCode objects do not have fields")
    }
    fn size(&self) -> Option<usize> {
        None
    }
}










pub enum ByteCode {
    Halt,
    NoOp,
    AccessField(usize),
    AccessTemp(usize),
    PushLiteral(ObjectBox<dyn Object>),
    AccessClass(String),
    StoreField(usize),
    StoreTemp(usize),
    /// Send a message to an object
    /// The first usize is the number of arguments to send
    /// The second usize is the index of the message to send
    SendMsg(usize, usize),
    SendSuperMsg(usize, usize),
    SpecialInstruction(SpecialInstruction),
}



pub enum SpecialInstruction {
    DupStack,
    DiscardStack,
    ReturnStack,
}
