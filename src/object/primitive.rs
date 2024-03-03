use super::{Class, Method, ObjectBox};
use std::{cell::RefCell, rc::Rc};
use crate::object::Object;
use super::Context;
use std::sync::Arc;
use super::Fault;
use num_traits::Zero;



macro_rules! create_type_ops {
    ($type:ty, $add_name:ident, $sub_name:ident, $mul_name:ident, $div_name:ident, $mod_name:ident) => {

        fn $add_name(object: ObjectBox<dyn Object>, context: &mut Context) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            {
                let mut object = object.borrow_mut();
                let object = object.downcast_mut::<PrimitiveObject<$type>>();
                let original_arg = context.arguments[0].clone();
                let mut arg_mut = original_arg.borrow_mut();
                match object {
                    Some(object) => {
                        if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<i64>>() {
                            object.data += arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<i32>>() {
                            object.data += arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<i16>>() {
                            object.data += arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<i8>>() {
                            object.data += arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<u64>>() {
                            object.data += arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<u32>>() {
                            object.data += arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<u16>>() {
                            object.data += arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<u8>>() {
                            object.data += arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_mut::<PrimitiveObject<f64>>() {
                            arg.data += object.data as f64;
                            drop(arg_mut);
                            return Ok(Some(original_arg));
                        } else if let Some(arg) = arg_mut.downcast_mut::<PrimitiveObject<f32>>() {
                            arg.data += object.data as f32;
                            drop(arg_mut);
                            return Ok(Some(original_arg));
                                            
                        } else {
                            return Err(Fault::InvalidType);
                        }
                    }
                    _ => {
                        return Err(Fault::InvalidType);
                    }
                }
            }
            Ok(Some(object))
        }

        fn $sub_name(object: ObjectBox<dyn Object>, context: &mut Context) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            {
                let mut object = object.borrow_mut();
                let object = object.downcast_mut::<PrimitiveObject<$type>>();
                let original_arg = context.arguments[0].clone();
                let mut arg_mut = original_arg.borrow_mut();
                match object {
                    Some(object) => {
                        if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<i64>>() {
                            object.data -= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<i32>>() {
                            object.data -= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<i16>>() {
                            object.data -= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<i8>>() {
                            object.data -= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<u64>>() {
                            object.data -= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<u32>>() {
                            object.data -= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<u16>>() {
                            object.data -= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<u8>>() {
                            object.data -= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_mut::<PrimitiveObject<f64>>() {
                            arg.data -= object.data as f64;
                            drop(arg_mut);
                            return Ok(Some(original_arg));
                        } else if let Some(arg) = arg_mut.downcast_mut::<PrimitiveObject<f32>>() {
                            arg.data -= object.data as f32;
                            drop(arg_mut);
                            return Ok(Some(original_arg));
                                            
                        } else {
                            return Err(Fault::InvalidType);
                        }
                    }
                    _ => {
                        return Err(Fault::InvalidType);
                    }
                }
            }
            Ok(Some(object))
        }
        

        fn $mul_name(object: ObjectBox<dyn Object>, context: &mut Context) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            {
                let mut object = object.borrow_mut();
                let object = object.downcast_mut::<PrimitiveObject<$type>>();
                let original_arg = context.arguments[0].clone();
                let mut arg_mut = original_arg.borrow_mut();
                match object {
                    Some(object) => {
                        if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<i64>>() {
                            object.data *= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<i32>>() {
                            object.data *= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<i16>>() {
                            object.data *= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<i8>>() {
                            object.data *= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<u64>>() {
                            object.data *= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<u32>>() {
                            object.data *= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<u16>>() {
                            object.data *= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<u8>>() {
                            object.data *= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_mut::<PrimitiveObject<f64>>() {
                            arg.data *= object.data as f64;
                            drop(arg_mut);
                            return Ok(Some(original_arg));
                        } else if let Some(arg) = arg_mut.downcast_mut::<PrimitiveObject<f32>>() {
                            arg.data *= object.data as f32;
                            drop(arg_mut);
                            return Ok(Some(original_arg));
                                            
                        } else {
                            return Err(Fault::InvalidType);
                        }
                    }
                    _ => {
                        return Err(Fault::InvalidType);
                    }
                }
            }
            Ok(Some(object))
        }

        fn $div_name(object: ObjectBox<dyn Object>, context: &mut Context) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            {
                let mut object = object.borrow_mut();
                let object = object.downcast_mut::<PrimitiveObject<$type>>();
                let original_arg = context.arguments[0].clone();
                let mut arg_mut = original_arg.borrow_mut();
                match object {
                    Some(object) => {
                        if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<i64>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data /= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<i32>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data /= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<i16>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data /= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<i8>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data /= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<u64>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data /= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<u32>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data /= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<u16>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data /= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<u8>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data /= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_mut::<PrimitiveObject<f64>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            arg.data = object.data as f64 / arg.data;
                            drop(arg_mut);
                            return Ok(Some(original_arg));
                        } else if let Some(arg) = arg_mut.downcast_mut::<PrimitiveObject<f32>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            arg.data = object.data as f32 / arg.data;
                            drop(arg_mut);
                            return Ok(Some(original_arg));
                        } else {
                            return Err(Fault::InvalidType);
                        }
                    }
                    _ => {
                        return Err(Fault::InvalidType);
                    }
                }
            }
            Ok(Some(object))
        }

        fn $mod_name(object: ObjectBox<dyn Object>, context: &mut Context) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            {
                let mut object = object.borrow_mut();
                let object = object.downcast_mut::<PrimitiveObject<$type>>();
                let original_arg = context.arguments[0].clone();
                let mut arg_mut = original_arg.borrow_mut();
                match object {
                    Some(object) => {
                        if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<i64>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data %= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<i32>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data %= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<i16>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data %= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<i8>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data %= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<u64>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data %= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<u32>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data %= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<u16>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data %= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<PrimitiveObject<u8>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data %= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_mut::<PrimitiveObject<f64>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            arg.data = object.data as f64 % arg.data;
                            drop(arg_mut);
                            return Ok(Some(original_arg));
                        } else if let Some(arg) = arg_mut.downcast_mut::<PrimitiveObject<f32>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            arg.data = object.data as f32 % arg.data;
                            drop(arg_mut);
                            return Ok(Some(original_arg));
                        } else {
                            return Err(Fault::InvalidType);
                        }
                    }
                    _ => {
                        return Err(Fault::InvalidType);
                    }
                }
            }
            Ok(Some(object))
        }
    };
}



pub struct PrimitiveObject<T: 'static> {
    class: Class,
    super_object: Option<ObjectBox<dyn Object>>,
    pub data: T,
}

impl<T: 'static> PrimitiveObject<T> {
    pub fn new(class: Class, super_object: Option<ObjectBox<dyn Object>>, data: T) -> Self {
        Self { class, super_object, data }
    }
}

impl <T> Object for PrimitiveObject<T> {
    fn get_class(&self) -> &Class {
        &self.class
    }
    fn get_super_object(&self) -> Option<ObjectBox<dyn Object>> {
        self.super_object.clone()
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox<dyn Object>> {
        panic!("Primitive objects do not have fields")
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox<dyn Object>) {
        panic!("Primitive objects do not have fields")
    }
    fn size(&self) -> Option<usize> {
        None
    }
}


pub struct NumberObject {
    class: Class,
    super_object: Option<ObjectBox<dyn Object>>,
}

impl NumberObject {
    pub fn make_class(parent: Box<Class>) -> Class {
        let mut methods = Vec::new();
        methods.push(Arc::new(Method::RustMethod { fun: Box::new(number_add) }));
        methods.push(Arc::new(Method::RustMethod { fun: Box::new(number_subtract) }));
        methods.push(Arc::new(Method::RustMethod { fun: Box::new(number_multiply) }));
        methods.push(Arc::new(Method::RustMethod { fun: Box::new(number_divide) }));
        methods.push(Arc::new(Method::RustMethod { fun: Box::new(number_modulo) }));
        
        Class::new(Some(parent), methods)
    }
    pub fn make_object(class: Class,
                           parent: ObjectBox<dyn Object>) -> ObjectBox<dyn Object> {
        Rc::new(RefCell::new(NumberObject {class, super_object: Some(parent)})) as ObjectBox<dyn Object>
    }
}

impl Object for NumberObject {
    fn get_class(&self) -> &Class {
        &self.class
    }
    fn get_super_object(&self) -> Option<ObjectBox<dyn Object>> {
        self.super_object.clone()
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox<dyn Object>> {
        panic!("Number objects do not have fields")
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox<dyn Object>) {
        panic!("Number objects do not have fields")
    }
    fn size(&self) -> Option<usize> {
        None
    }
}

fn number_add(_: ObjectBox<dyn Object>, _: &mut Context) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn number_subtract(_: ObjectBox<dyn Object>, _: &mut Context) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn number_multiply(_: ObjectBox<dyn Object>, _: &mut Context) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn number_divide(_: ObjectBox<dyn Object>, _: &mut Context) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn number_modulo(_: ObjectBox<dyn Object>, _: &mut Context) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}


pub struct I64Object {
}

impl I64Object {
    pub fn make_class(mut parent: Box<Class>) -> Class {
        let methods = Vec::new();
        let parent_mut = parent.as_mut();
        parent_mut.override_method(0, Arc::new(Method::RustMethod { fun: Box::new(i64_add) }));
        parent_mut.override_method(1, Arc::new(Method::RustMethod { fun: Box::new(i64_sub) }));
        parent_mut.override_method(2, Arc::new(Method::RustMethod { fun: Box::new(i64_mul) }));
        parent_mut.override_method(3, Arc::new(Method::RustMethod { fun: Box::new(i64_div) }));
        parent_mut.override_method(4, Arc::new(Method::RustMethod { fun: Box::new(i64_mod) }));

        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Class,
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
    pub fn make_class(mut parent: Box<Class>) -> Class {
        let methods = Vec::new();
        let parent_mut = parent.as_mut();
        parent_mut.override_method(0, Arc::new(Method::RustMethod { fun: Box::new(u64_add) }));
        parent_mut.override_method(1, Arc::new(Method::RustMethod { fun: Box::new(u64_sub) }));
        parent_mut.override_method(2, Arc::new(Method::RustMethod { fun: Box::new(u64_mul) }));
        parent_mut.override_method(3, Arc::new(Method::RustMethod { fun: Box::new(u64_div) }));
        parent_mut.override_method(4, Arc::new(Method::RustMethod { fun: Box::new(u64_mod) }));

        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Class,
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
    pub fn make_class(mut parent: Box<Class>) -> Class {
        let methods = Vec::new();
        let parent_mut = parent.as_mut();
        parent_mut.override_method(0, Arc::new(Method::RustMethod { fun: Box::new(i32_add) }));
        parent_mut.override_method(1, Arc::new(Method::RustMethod { fun: Box::new(i32_sub) }));
        parent_mut.override_method(2, Arc::new(Method::RustMethod { fun: Box::new(i32_mul) }));
        parent_mut.override_method(3, Arc::new(Method::RustMethod { fun: Box::new(i32_div) }));
        parent_mut.override_method(4, Arc::new(Method::RustMethod { fun: Box::new(i32_mod) }));

        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Class,
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
    pub fn make_class(mut parent: Box<Class>) -> Class {
        let methods = Vec::new();
        let parent_mut = parent.as_mut();
        parent_mut.override_method(0, Arc::new(Method::RustMethod { fun: Box::new(u32_add) }));
        parent_mut.override_method(1, Arc::new(Method::RustMethod { fun: Box::new(u32_sub) }));
        parent_mut.override_method(2, Arc::new(Method::RustMethod { fun: Box::new(u32_mul) }));
        parent_mut.override_method(3, Arc::new(Method::RustMethod { fun: Box::new(u32_div) }));
        parent_mut.override_method(4, Arc::new(Method::RustMethod { fun: Box::new(u32_mod) }));

        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Class,
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
    pub fn make_class(mut parent: Box<Class>) -> Class {
        let methods = Vec::new();
        let parent_mut = parent.as_mut();
        parent_mut.override_method(0, Arc::new(Method::RustMethod { fun: Box::new(i16_add) }));
        parent_mut.override_method(1, Arc::new(Method::RustMethod { fun: Box::new(i16_sub) }));
        parent_mut.override_method(2, Arc::new(Method::RustMethod { fun: Box::new(i16_mul) }));
        parent_mut.override_method(3, Arc::new(Method::RustMethod { fun: Box::new(i16_div) }));
        parent_mut.override_method(4, Arc::new(Method::RustMethod { fun: Box::new(i16_mod) }));

        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Class,
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
    pub fn make_class(mut parent: Box<Class>) -> Class {
        let methods = Vec::new();
        let parent_mut = parent.as_mut();
        parent_mut.override_method(0, Arc::new(Method::RustMethod { fun: Box::new(u16_add) }));
        parent_mut.override_method(1, Arc::new(Method::RustMethod { fun: Box::new(u16_sub) }));
        parent_mut.override_method(2, Arc::new(Method::RustMethod { fun: Box::new(u16_mul) }));
        parent_mut.override_method(3, Arc::new(Method::RustMethod { fun: Box::new(u16_div) }));
        parent_mut.override_method(4, Arc::new(Method::RustMethod { fun: Box::new(u16_mod) }));

        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Class,
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
    pub fn make_class(mut parent: Box<Class>) -> Class {
        let methods = Vec::new();
        let parent_mut = parent.as_mut();
        parent_mut.override_method(0, Arc::new(Method::RustMethod { fun: Box::new(i8_add) }));
        parent_mut.override_method(1, Arc::new(Method::RustMethod { fun: Box::new(i8_sub) }));
        parent_mut.override_method(2, Arc::new(Method::RustMethod { fun: Box::new(i8_mul) }));
        parent_mut.override_method(3, Arc::new(Method::RustMethod { fun: Box::new(i8_div) }));
        parent_mut.override_method(4, Arc::new(Method::RustMethod { fun: Box::new(i8_mod) }));

        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Class,
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
    pub fn make_class(mut parent: Box<Class>) -> Class {
        let methods = Vec::new();
        let parent_mut = parent.as_mut();
        parent_mut.override_method(0, Arc::new(Method::RustMethod { fun: Box::new(u8_add) }));
        parent_mut.override_method(1, Arc::new(Method::RustMethod { fun: Box::new(u8_sub) }));
        parent_mut.override_method(2, Arc::new(Method::RustMethod { fun: Box::new(u8_mul) }));
        parent_mut.override_method(3, Arc::new(Method::RustMethod { fun: Box::new(u8_div) }));
        parent_mut.override_method(4, Arc::new(Method::RustMethod { fun: Box::new(u8_mod) }));

        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Class,
                           parent: ObjectBox<dyn Object>,
                           data: u8) -> ObjectBox<dyn Object> {
        let out = Rc::new(RefCell::new(PrimitiveObject::new(class, Some(parent), data)));
        return out as ObjectBox<dyn Object>;
    }
}

create_type_ops!(u8, u8_add, u8_sub, u8_mul, u8_div, u8_mod);


pub struct F64Object {
}

impl F64Object {
    pub fn make_class(mut parent: Box<Class>) -> Class {
        let methods = Vec::new();
        let parent_mut = parent.as_mut();
        parent_mut.override_method(0, Arc::new(Method::RustMethod { fun: Box::new(f64_add) }));
        parent_mut.override_method(1, Arc::new(Method::RustMethod { fun: Box::new(f64_sub) }));
        parent_mut.override_method(2, Arc::new(Method::RustMethod { fun: Box::new(f64_mul) }));
        parent_mut.override_method(3, Arc::new(Method::RustMethod { fun: Box::new(f64_div) }));
        parent_mut.override_method(4, Arc::new(Method::RustMethod { fun: Box::new(f64_mod) }));

        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Class,
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
    pub fn make_class(mut parent: Box<Class>) -> Class {
        let methods = Vec::new();
        let parent_mut = parent.as_mut();
        parent_mut.override_method(0, Arc::new(Method::RustMethod { fun: Box::new(f32_add) }));
        parent_mut.override_method(1, Arc::new(Method::RustMethod { fun: Box::new(f32_sub) }));
        parent_mut.override_method(2, Arc::new(Method::RustMethod { fun: Box::new(f32_mul) }));
        parent_mut.override_method(3, Arc::new(Method::RustMethod { fun: Box::new(f32_div) }));
        parent_mut.override_method(4, Arc::new(Method::RustMethod { fun: Box::new(f32_mod) }));
        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Class,
                           parent: ObjectBox<dyn Object>,
                           data: f32) -> ObjectBox<dyn Object> {
        let out = Rc::new(RefCell::new(PrimitiveObject::new(class, Some(parent), data)));
        return out as ObjectBox<dyn Object>;
    }
}

create_type_ops!(f32, f32_add, f32_sub, f32_mul, f32_div, f32_mod);
