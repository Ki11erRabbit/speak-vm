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
use crate::Interpreter;



pub struct IntegerObject {
    class: Arc<Class>,
    super_object: Option<ObjectBox<dyn Object>>,
}

impl IntegerObject {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let mut methods = HashMap::new();
        methods.insert(String::from("divides"), Arc::new(Method::RustMethod { fun: Box::new(integer_divides) }));
        methods.insert(String::from("shift_right"), Arc::new(Method::RustMethod { fun: Box::new(integer_shift_right) }));
        methods.insert(String::from("shift_left"), Arc::new(Method::RustMethod { fun: Box::new(integer_shift_left) }));
        methods.insert(String::from("and"), Arc::new(Method::RustMethod { fun: Box::new(integer_bitwise_and) }));
        methods.insert(String::from("or"), Arc::new(Method::RustMethod { fun: Box::new(integer_bitwise_or) }));
        methods.insert(String::from("xor"), Arc::new(Method::RustMethod { fun: Box::new(integer_bitwise_xor) }));
        
        Class::new(Some(parent), methods)
    }
    pub fn make_object(class: Arc<Class>,
                       parent: ObjectBox<dyn Object>) -> ObjectBox<dyn Object> {
        let out = Rc::new(RefCell::new(IntegerObject{class, super_object: Some(parent)}));
        return out as ObjectBox<dyn Object>;
    }
}


impl Object for IntegerObject {
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

fn integer_divides(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn integer_shift_right(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn integer_shift_left(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn integer_bitwise_and(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn integer_bitwise_or(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn integer_bitwise_xor(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}


pub struct I64Object {
}

impl I64Object {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let mut methods = HashMap::new();
        methods.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(i64_add) }));
        methods.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(i64_sub) }));
        methods.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(i64_mul) }));
        methods.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(i64_div) }));
        methods.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(i64_mod) }));


        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Arc<Class>,
                           parent: ObjectBox<dyn Object>,
                           data: i64) -> ObjectBox<dyn Object> {
        let out = Rc::new(RefCell::new(PrimitiveObject::new(class, Some(parent), data)));
        return out as ObjectBox<dyn Object>;
    }
}

create_type_ops!(i64, i64_add, i64_sub, i64_mul, i64_div, i64_mod);


pub struct U64Object {
}

impl U64Object {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let mut methods = HashMap::new();
        methods.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(u64_add) }));
        methods.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(u64_sub) }));
        methods.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(u64_mul) }));
        methods.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(u64_div) }));
        methods.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(u64_mod) }));

        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Arc<Class>,
                           parent: ObjectBox<dyn Object>,
                           data: u64) -> ObjectBox<dyn Object> {
        let out = Rc::new(RefCell::new(PrimitiveObject::new(class, Some(parent), data)));
        return out as ObjectBox<dyn Object>;
    }
}

create_type_ops!(u64, u64_add, u64_sub, u64_mul, u64_div, u64_mod);

pub struct I32Object {
}

impl I32Object {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let mut methods = HashMap::new();
        methods.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(i32_add) }));
        methods.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(i32_sub) }));
        methods.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(i32_mul) }));
        methods.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(i32_div) }));
        methods.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(i32_mod) }));

        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Arc<Class>,
                           parent: ObjectBox<dyn Object>,
                           data: i32) -> ObjectBox<dyn Object> {
        let out = Rc::new(RefCell::new(PrimitiveObject::new(class, Some(parent), data)));
        return out as ObjectBox<dyn Object>;
    }
}

create_type_ops!(i32, i32_add, i32_sub, i32_mul, i32_div, i32_mod);


pub struct U32Object {
}

impl U32Object {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let mut methods = HashMap::new();
        methods.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(u32_add) }));
        methods.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(u32_sub) }));
        methods.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(u32_mul) }));
        methods.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(u32_div) }));
        methods.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(u32_mod) }));

        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Arc<Class>,
                           parent: ObjectBox<dyn Object>,
                           data: u32) -> ObjectBox<dyn Object> {
        let out = Rc::new(RefCell::new(PrimitiveObject::new(class, Some(parent), data)));
        return out as ObjectBox<dyn Object>;
    }
}

create_type_ops!(u32, u32_add, u32_sub, u32_mul, u32_div, u32_mod);


pub struct I16Object {
}

impl I16Object {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let mut methods = HashMap::new();
        methods.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(i16_add) }));
        methods.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(i16_sub) }));
        methods.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(i16_mul) }));
        methods.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(i16_div) }));
        methods.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(i16_mod) }));

        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Arc<Class>,
                           parent: ObjectBox<dyn Object>,
                           data: i16) -> ObjectBox<dyn Object> {
        let out = Rc::new(RefCell::new(PrimitiveObject::new(class, Some(parent), data)));
        return out as ObjectBox<dyn Object>;
    }
}

create_type_ops!(i16, i16_add, i16_sub, i16_mul, i16_div, i16_mod);


pub struct U16Object {
}

impl U16Object {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let mut methods = HashMap::new();
        methods.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(u16_add) }));
        methods.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(u16_sub) }));
        methods.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(u16_mul) }));
        methods.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(u16_div) }));
        methods.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(u16_mod) }));

        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Arc<Class>,
                           parent: ObjectBox<dyn Object>,
                           data: u16) -> ObjectBox<dyn Object> {
        let out = Rc::new(RefCell::new(PrimitiveObject::new(class, Some(parent), data)));
        return out as ObjectBox<dyn Object>;
    }
}

create_type_ops!(u16, u16_add, u16_sub, u16_mul, u16_div, u16_mod);


pub struct I8Object {
}

impl I8Object {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let mut methods = HashMap::new();
        methods.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(i8_add) }));
        methods.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(i8_sub) }));
        methods.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(i8_mul) }));
        methods.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(i8_div) }));
        methods.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(i8_mod) }));

        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Arc<Class>,
                           parent: ObjectBox<dyn Object>,
                           data: i8) -> ObjectBox<dyn Object> {
        let out = Rc::new(RefCell::new(PrimitiveObject::new(class, Some(parent), data)));
        return out as ObjectBox<dyn Object>;
    }
}

create_type_ops!(i8, i8_add, i8_sub, i8_mul, i8_div, i8_mod);

pub struct U8Object {
}

impl U8Object {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let mut methods = HashMap::new();
        methods.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(u8_add) }));
        methods.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(u8_sub) }));
        methods.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(u8_mul) }));
        methods.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(u8_div) }));
        methods.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(u8_mod) }));

        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Arc<Class>,
                           parent: ObjectBox<dyn Object>,
                           data: u8) -> ObjectBox<dyn Object> {
        let out = Rc::new(RefCell::new(PrimitiveObject::new(class, Some(parent), data)));
        return out as ObjectBox<dyn Object>;
    }
}

create_type_ops!(u8, u8_add, u8_sub, u8_mul, u8_div, u8_mod);


