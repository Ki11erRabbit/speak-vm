use std::sync::Arc;
use crate::object::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::object::primitive::PrimitiveObject;
use crate::object::ObjectBox;
use crate::object::Class;








pub struct CharacterObject {}

impl CharacterObject {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let methods = HashMap::new();
        Class::new(Some(parent), methods)
    }
    pub fn make_object(class: Arc<Class>, parent: ObjectBox, value: char) -> ObjectBox {
        Rc::new(RefCell::new(PrimitiveObject::new(class, Some(parent), value)))
    }
}

       
