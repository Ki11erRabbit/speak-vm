use crate::object::Interpreter;
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

pub struct FloatObject {
    class: Arc<Class>,
    super_object: Option<ObjectBox<dyn Object>>,
}

impl FloatObject {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let mut methods = HashMap::new();
        methods.insert(String::from("is_nan"), Arc::new(Method::RustMethod { fun: Box::new(float_is_nan) }));
        methods.insert(String::from("is_infinity"), Arc::new(Method::RustMethod { fun: Box::new(float_is_infinity) }));
        methods.insert(String::from("is_neg_infinity"), Arc::new(Method::RustMethod { fun: Box::new(float_is_neg_infinity) }));
        methods.insert(String::from("is_finite"), Arc::new(Method::RustMethod { fun: Box::new(float_is_finite) }));
        methods.insert(String::from("is_normal"), Arc::new(Method::RustMethod { fun: Box::new(float_is_normal) }));
        methods.insert(String::from("floor"), Arc::new(Method::RustMethod { fun: Box::new(float_floor) }));
        methods.insert(String::from("ceil"), Arc::new(Method::RustMethod { fun: Box::new(float_ceil) }));
        methods.insert(String::from("nat_log"), Arc::new(Method::RustMethod { fun: Box::new(float_nat_log) }));
        methods.insert(String::from("log"), Arc::new(Method::RustMethod { fun: Box::new(float_log) }));
        methods.insert(String::from("hypotenuse"), Arc::new(Method::RustMethod { fun: Box::new(float_hypotenuse) }));
        methods.insert(String::from("sin"), Arc::new(Method::RustMethod { fun: Box::new(float_sin) }));
        methods.insert(String::from("cos"), Arc::new(Method::RustMethod { fun: Box::new(float_cos) }));
        methods.insert(String::from("tan"), Arc::new(Method::RustMethod { fun: Box::new(float_tan) }));
        methods.insert(String::from("arcsin"), Arc::new(Method::RustMethod { fun: Box::new(float_arcsin) }));
        methods.insert(String::from("arccos"), Arc::new(Method::RustMethod { fun: Box::new(float_arccos) }));
        methods.insert(String::from("arctan"), Arc::new(Method::RustMethod { fun: Box::new(float_arctan) }));
        
        Class::new(Some(parent), methods)
    }
    pub fn make_object(class: Arc<Class>,
                           parent: ObjectBox<dyn Object>) -> ObjectBox<dyn Object> {
        let out = Rc::new(RefCell::new(FloatObject {class, super_object: Some(parent)}));
        return out as ObjectBox<dyn Object>;
    }
}

impl Object for FloatObject {
    fn get_class(&self) -> Arc<Class> {
        self.class.clone()
    }
    fn get_super_object(&self) -> Option<ObjectBox<dyn Object>> {
        self.super_object.clone()
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox<dyn Object>> {
        panic!("Integer objects do not have fields")
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox<dyn Object>) {
        panic!("Integer objects do not have fields")
    }
    fn size(&self) -> Option<usize> {
        None
    }
}

fn float_is_nan(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_is_infinity(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_is_neg_infinity(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_is_finite(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_is_normal(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_floor(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_ceil(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_nat_log(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_log(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_hypotenuse(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_sin(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_cos(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_tan(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_arcsin(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_arccos(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_arctan(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}





pub struct F64Object {
}

impl F64Object {
    pub fn make_class(parent: Arc<Class>) -> Class {
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
    pub fn make_class(parent: Arc<Class>) -> Class {
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
