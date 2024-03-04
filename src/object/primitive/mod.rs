use super::{Class, Method, ObjectBox};
use crate::object::Object;
use super::Context;
use super::Fault;
use std::collections::HashMap;
use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;
use crate::object::Interpreter;


pub mod integer;
pub mod float;
pub mod boolean;
pub mod character;

#[derive(Clone)]
pub struct PrimitiveObject<T: Copy + 'static> {
    class: Arc<Class>,
    super_object: Option<ObjectBox<dyn Object>>,
    pub data: T,
}

impl<T: Copy + 'static> PrimitiveObject<T> {
    pub fn new(class: Arc<Class>, super_object: Option<ObjectBox<dyn Object>>, data: T) -> Self {
        Self { class, super_object, data }
    }
}

impl<T: Copy> Object for PrimitiveObject<T> {
    fn get_class(&self) -> Arc<Class> {
        self.class.clone()
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
    fn duplicate(&self) -> ObjectBox<dyn Object> {
        Rc::new(RefCell::new(PrimitiveObject::new(self.class.clone(), self.super_object.clone(), self.data)))
    }
}



pub struct NumberObject {
    class: Arc<Class>,
    super_object: Option<ObjectBox<dyn Object>>,
}

impl NumberObject {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let mut methods = HashMap::new();
        methods.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(number_add) }));
        methods.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(number_subtract) }));
        methods.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(number_multiply) }));
        methods.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(number_divide) }));
        methods.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(number_modulo) }));
        methods.insert(String::from("abs"), Arc::new(Method::RustMethod { fun: Box::new(number_abs) }));
        methods.insert(String::from("pow"), Arc::new(Method::RustMethod { fun: Box::new(number_pow) }));
        methods.insert(String::from("is_zero"), Arc::new(Method::RustMethod { fun: Box::new(number_is_zero) }));
        
        Class::new(Some(parent), methods)
    }
    pub fn make_object(class: Arc<Class>,
                           parent: ObjectBox<dyn Object>) -> ObjectBox<dyn Object> {
        Rc::new(RefCell::new(NumberObject {class, super_object: Some(parent)})) as ObjectBox<dyn Object>
    }
}

impl Object for NumberObject {
    fn get_class(&self) -> Arc<Class> {
        self.class.clone()
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
    fn duplicate(&self) -> ObjectBox<dyn Object> {
        Rc::new(RefCell::new(NumberObject {class: self.class.clone(), super_object: self.super_object.clone()}))
    }
}

fn number_add(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn number_subtract(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn number_multiply(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn number_divide(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn number_modulo(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn number_abs(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn number_pow(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn number_is_zero(_: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}


#[macro_export]
macro_rules! create_type_ops {
    ($type:ty, $add_name:ident, $sub_name:ident, $mul_name:ident, $div_name:ident, $mod_name:ident) => {

        fn $add_name(object: ObjectBox<dyn Object>, context: &mut Context, _: &mut crate::object::Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            {
                let mut object = object.borrow_mut();
                let object = object.downcast_mut::<crate::object::primitive::PrimitiveObject<$type>>();
                let original_arg = context.arguments[0].clone();
                let mut arg_mut = original_arg.borrow_mut();
                match object {
                    Some(object) => {
                        if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i64>>() {
                            object.data += arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i32>>() {
                            object.data += arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i16>>() {
                            object.data += arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i8>>() {
                            object.data += arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u64>>() {
                            object.data += arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u32>>() {
                            object.data += arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u16>>() {
                            object.data += arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u8>>() {
                            object.data += arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_mut::<crate::object::primitive::PrimitiveObject<f64>>() {
                            arg.data += object.data as f64;
                            drop(arg_mut);
                            return Ok(Some(original_arg));
                        } else if let Some(arg) = arg_mut.downcast_mut::<crate::object::primitive::PrimitiveObject<f32>>() {
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
            Ok(None)
        }

        fn $sub_name(object: ObjectBox<dyn Object>, context: &mut Context, _: &mut crate::object::Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            {
                let mut object = object.borrow_mut();
                let object = object.downcast_mut::<crate::object::primitive::PrimitiveObject<$type>>();
                let original_arg = context.arguments[0].clone();
                let mut arg_mut = original_arg.borrow_mut();
                match object {
                    Some(object) => {
                        if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i64>>() {
                            object.data -= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i32>>() {
                            object.data -= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i16>>() {
                            object.data -= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i8>>() {
                            object.data -= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u64>>() {
                            object.data -= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u32>>() {
                            object.data -= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u16>>() {
                            object.data -= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u8>>() {
                            object.data -= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_mut::<crate::object::primitive::PrimitiveObject<f64>>() {
                            arg.data -= object.data as f64;
                            drop(arg_mut);
                            return Ok(Some(original_arg));
                        } else if let Some(arg) = arg_mut.downcast_mut::<crate::object::primitive::PrimitiveObject<f32>>() {
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
            Ok(None)
        }
        

        fn $mul_name(object: ObjectBox<dyn Object>, context: &mut Context, _: &mut crate::object::Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            {
                let mut object = object.borrow_mut();
                let object = object.downcast_mut::<crate::object::primitive::PrimitiveObject<$type>>();
                let original_arg = context.arguments[0].clone();
                let mut arg_mut = original_arg.borrow_mut();
                match object {
                    Some(object) => {
                        if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i64>>() {
                            object.data *= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i32>>() {
                            object.data *= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i16>>() {
                            object.data *= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i8>>() {
                            object.data *= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u64>>() {
                            object.data *= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u32>>() {
                            object.data *= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u16>>() {
                            object.data *= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u8>>() {
                            object.data *= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_mut::<crate::object::primitive::PrimitiveObject<f64>>() {
                            arg.data *= object.data as f64;
                            drop(arg_mut);
                            return Ok(Some(original_arg));
                        } else if let Some(arg) = arg_mut.downcast_mut::<crate::object::primitive::PrimitiveObject<f32>>() {
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
            Ok(None)
        }

        fn $div_name(object: ObjectBox<dyn Object>, context: &mut Context, _: &mut crate::object::Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            {
                let mut object = object.borrow_mut();
                let object = object.downcast_mut::<crate::object::primitive::PrimitiveObject<$type>>();
                let original_arg = context.arguments[0].clone();
                let mut arg_mut = original_arg.borrow_mut();
                match object {
                    Some(object) => {
                        if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i64>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data /= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i32>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data /= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i16>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data /= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i8>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data /= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u64>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data /= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u32>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data /= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u16>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data /= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u8>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data /= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_mut::<crate::object::primitive::PrimitiveObject<f64>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            arg.data = object.data as f64 / arg.data;
                            drop(arg_mut);
                            return Ok(Some(original_arg));
                        } else if let Some(arg) = arg_mut.downcast_mut::<crate::object::primitive::PrimitiveObject<f32>>() {
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
            Ok(None)
        }

        fn $mod_name(object: ObjectBox<dyn Object>, context: &mut Context, _: &mut crate::object::Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            {
                let mut object = object.borrow_mut();
                let object = object.downcast_mut::<crate::object::primitive::PrimitiveObject<$type>>();
                let original_arg = context.arguments[0].clone();
                let mut arg_mut = original_arg.borrow_mut();
                match object {
                    Some(object) => {
                        if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i64>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data %= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i32>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data %= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i16>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data %= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i8>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data %= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u64>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data %= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u32>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data %= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u16>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data %= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u8>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            object.data %= arg.data as $type;
                        } else if let Some(arg) = arg_mut.downcast_mut::<crate::object::primitive::PrimitiveObject<f64>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            arg.data = object.data as f64 % arg.data;
                            drop(arg_mut);
                            return Ok(Some(original_arg));
                        } else if let Some(arg) = arg_mut.downcast_mut::<crate::object::primitive::PrimitiveObject<f32>>() {
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
            Ok(None)
        }
    };
}
