use std::cell::RefCell;
use std::{collections::HashMap, rc::Rc};
use std::sync::Arc;
use crate::object::{Class, Object, ObjectBox};

use super::PrimitiveObject;








pub struct BooleanObject {}


impl BooleanObject {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let methods = HashMap::new();
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Arc<Class>, parent: ObjectBox<dyn Object>, data: bool) -> ObjectBox<dyn Object> {
        Rc::new(RefCell::new(PrimitiveObject::new(class, Some(parent), data)))
    }
}









