use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

use super::{Class, Object, ObjectBox};





pub struct StringObject {
    class: Arc<Class>,
    super_object: Option<ObjectBox>,
    pub value: String,
}



impl StringObject {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let methods = HashMap::new();
        Class::new(Some(parent), methods)
    }
    pub fn make_object(class: Arc<Class>, parent: ObjectBox, value: String) -> ObjectBox {
        let string = StringObject {
            class,
            super_object: Some(parent),
            value,
        };
        Rc::new(RefCell::new(string)) as ObjectBox
    }
}


impl Object for StringObject {
    fn get_class(&self) -> Arc<Class> { 
        self.class.clone()
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        self.super_object.clone()
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox> {
        panic!("String does not have fields");
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox) {
        panic!("String does not have fields");
    }
    fn size(&self) -> Option<usize> {
        Some(self.value.len())
    }
    fn duplicate(&self) -> ObjectBox {
        StringObject::make_object(self.class.clone(), self.super_object.clone().unwrap(), self.value.clone())
    }
    fn initalize(&mut self, _: Vec<ObjectBox>) {
        panic!("String objects do not take arguments");
    }
}










