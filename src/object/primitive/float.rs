
use super::{Class, Method, ObjectBox};
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};
use crate::object::Object;
use super::Context;
use std::sync::Arc;
use super::Fault;
use num_traits::Zero;
use crate::create_type_ops;
use crate::object::primitive::PrimitiveObject;

pub struct F64Object {
}

impl F64Object {
    pub fn make_class(mut parent: Arc<Class>) -> Class {
        let mut methods = HashMap::new();
        methods.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(f64_add) }));
        methods.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(f64_sub) }));
        methods.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(f64_mul) }));
        methods.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(f64_div) }));
        methods.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(f64_mod) }));

        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Arc<Class>,
                           parent: ObjectBox<dyn Object>,
                           data: f64) -> ObjectBox<dyn Object> {
        let out = Rc::new(RefCell::new(PrimitiveObject::new(class, Some(parent), data)));
        return out as ObjectBox<dyn Object>;
    }
}

create_type_ops!(f64, f64_add, f64_sub, f64_mul, f64_div, f64_mod);

pub struct F32Object {
}

impl F32Object {
    pub fn make_class(mut parent: Arc<Class>) -> Class {
        let mut methods = HashMap::new();
        methods.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(f32_add) }));
        methods.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(f32_sub) }));
        methods.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(f32_mul) }));
        methods.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(f32_div) }));
        methods.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(f32_mod) }));
        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Arc<Class>,
                           parent: ObjectBox<dyn Object>,
                           data: f32) -> ObjectBox<dyn Object> {
        let out = Rc::new(RefCell::new(PrimitiveObject::new(class, Some(parent), data)));
        return out as ObjectBox<dyn Object>;
    }
}

create_type_ops!(f32, f32_add, f32_sub, f32_mul, f32_div, f32_mod);
