use super::{Method, ObjectBox, VTable};
use crate::object::Object;
use super::Fault;
use std::collections::HashMap;
use std::sync::Arc;
use super::ContextData;


pub mod integer;
pub mod float;
pub mod boolean;
pub mod character;

#[derive(Clone)]
pub struct PrimitiveObject<T: Copy + 'static> {
    super_object: Option<ObjectBox>,
    vtable: VTable,
    pub data: T,
}

impl<T: Copy + 'static> PrimitiveObject<T> {
    pub fn new(super_object: Option<ObjectBox>, data: T) -> Self {
        Self { super_object, data, vtable: VTable::new_empty() }
    }
}




pub struct NumberObject {
    super_object: Option<ObjectBox>,
    vtable: VTable,
}

impl NumberObject {
    pub fn make_object(parent: ObjectBox) -> ObjectBox {
        ObjectBox::new(NumberObject {super_object: Some(parent), vtable: VTable::new_empty()})
    }
    pub fn make_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(number_add) }));
        methods.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(number_subtract) }));
        methods.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(number_multiply) }));
        methods.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(number_divide) }));
        methods.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(number_modulo) }));
        methods.insert(String::from("abs"), Arc::new(Method::RustMethod { fun: Box::new(number_abs) }));
        methods.insert(String::from("pow"), Arc::new(Method::RustMethod { fun: Box::new(number_pow) }));
        methods.insert(String::from("is_zero"), Arc::new(Method::RustMethod { fun: Box::new(number_is_zero) }));
        
        VTable::new(methods)
    }
}

impl Object for NumberObject {
    fn get_super_object(&self) -> Option<ObjectBox> {
        self.super_object.clone()
    }
    fn get_vtable(&self) -> &VTable {
        &self.vtable
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox> {
        panic!("Number objects do not have fields")
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox) {
        panic!("Number objects do not have fields")
    }
    fn size(&self) -> Option<usize> {
        None
    }
    fn duplicate(&self) -> ObjectBox {
        let number = NumberObject {super_object: Some(self.super_object.clone().unwrap().borrow().duplicate()), vtable: self.vtable.clone()};
        ObjectBox::new(number)
    }
    fn initialize(&mut self, _: Vec<ObjectBox>, vtable: VTable) {
        self.vtable.extend(NumberObject::make_vtable());
        self.vtable.extend(vtable);
        match &mut self.super_object {
            Some(super_object) => {
                let mut super_object = super_object.borrow_mut();
                super_object.initialize(vec![], VTable::new_empty());
            }
            None => {}
        }
    }
}

fn number_add(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented(format!("Number add")))
}

fn number_subtract(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented(format!("Number sub")))
}

fn number_multiply(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented(format!("Number mul")))
}

fn number_divide(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented(format!("Number div")))
}

fn number_modulo(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented(format!("Number mod")))
}

fn number_abs(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented(format!("Number abs")))
}

fn number_pow(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented(format!("Number pow")))
}

fn number_is_zero(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented(format!("Number is_zero")))
}


#[macro_export]
macro_rules! create_type_ops {
    ($type:ty, $add_name:ident, $sub_name:ident, $mul_name:ident, $div_name:ident, $mod_name:ident, $abs:ident, $pow:ident, $is_zero:ident) => {

        fn $add_name(object: ObjectBox, context: &mut crate::object::ContextData) -> Result<Option<ObjectBox>, Fault> {
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
                            context.pop();
                            return Ok(Some(original_arg));
                        } else if let Some(arg) = arg_mut.downcast_mut::<crate::object::primitive::PrimitiveObject<f32>>() {
                            arg.data += object.data as f32;
                            drop(arg_mut);
                            context.pop();
                            return Ok(Some(original_arg));
                        } else {
                            return Err(Fault::InvalidType(format!("Number add: Not a number")));
                        }
                    }
                    _ => {
                        return Err(Fault::InvalidType(format!("Number add: expected {}", stringify!($type))));
                    }
                }
            }
            Ok(None)
        }

        fn $sub_name(object: ObjectBox, context: &mut crate::object::ContextData) -> Result<Option<ObjectBox>, Fault> {
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
                            context.pop();
                            return Ok(Some(original_arg));
                        } else if let Some(arg) = arg_mut.downcast_mut::<crate::object::primitive::PrimitiveObject<f32>>() {
                            arg.data -= object.data as f32;
                            drop(arg_mut);
                            context.pop();
                            return Ok(Some(original_arg));
                        } else {
                            return Err(Fault::InvalidType(format!("Number sub: Not a number")));
                        }
                    }
                    _ => {
                        return Err(Fault::InvalidType(format!("Number sub: expected {}", stringify!($type))));
                    }
                }
            }
            Ok(None)
        }
        

        fn $mul_name(object: ObjectBox, context: &mut crate::object::ContextData) -> Result<Option<ObjectBox>, Fault> {
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
                            context.pop();
                            return Ok(Some(original_arg));
                        } else if let Some(arg) = arg_mut.downcast_mut::<crate::object::primitive::PrimitiveObject<f32>>() {
                            arg.data *= object.data as f32;
                            drop(arg_mut);
                            context.pop();
                            return Ok(Some(original_arg));
                        } else {
                            return Err(Fault::InvalidType(format!("Number mul: Not a number")));
                        }
                    }
                    _ => {
                        return Err(Fault::InvalidType(format!("Number mul: expected {}", stringify!($type))));
                    }
                }
            }
            Ok(None)
        }

        fn $div_name(object: ObjectBox, context: &mut crate::object::ContextData) -> Result<Option<ObjectBox>, Fault> {
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
                            context.pop();
                            return Ok(Some(original_arg));
                        } else if let Some(arg) = arg_mut.downcast_mut::<crate::object::primitive::PrimitiveObject<f32>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            arg.data = object.data as f32 / arg.data;
                            drop(arg_mut);
                            context.pop();
                            return Ok(Some(original_arg));
                        } else {
                            return Err(Fault::InvalidType(format!("Number div: Not a number")));
                        }
                    }
                    _ => {
                        return Err(Fault::InvalidType(format!("Number add: expected {}", stringify!($type))));
                    }
                }
            }
            Ok(None)
        }

        fn $mod_name(object: ObjectBox, context: &mut crate::object::ContextData) -> Result<Option<ObjectBox>, Fault> {
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
                            context.pop();
                            return Ok(Some(original_arg));
                        } else if let Some(arg) = arg_mut.downcast_mut::<crate::object::primitive::PrimitiveObject<f32>>() {
                            if arg.data.is_zero() {
                                return Err(Fault::DivideByZero);
                            }
                            arg.data = object.data as f32 % arg.data;
                            drop(arg_mut);
                            context.pop();
                            return Ok(Some(original_arg));
                        } else {
                            return Err(Fault::InvalidType(format!("Number mod: Not a number")));
                        }
                    }
                    _ => {
                        return Err(Fault::InvalidType(format!("Number mod: expected {}", stringify!($type))));
                    }
                }
            }
            Ok(None)
        }
        
        fn $abs(object: ObjectBox, _: &mut crate::object::ContextData) -> Result<Option<ObjectBox>, Fault> {
            let mut object = object.borrow_mut();
            if let Some(object) = object.downcast_mut::<crate::object::primitive::PrimitiveObject<$type>>() {
                object.data = object.data.abs();
            } else {
                return Err(Fault::InvalidType(format!("Number abs: expected {}", stringify!($type))))
            }
            Ok(None)
        }

        fn $pow(object: ObjectBox, context: &mut crate::object::ContextData) -> Result<Option<ObjectBox>, Fault> {
            {
                let mut object = object.borrow_mut();
                let object = object.downcast_mut::<crate::object::primitive::PrimitiveObject<$type>>();
                let original_arg = context.arguments[0].clone();
                let mut arg_mut = original_arg.borrow_mut();
                match object {
                    Some(object) => {
                        if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i64>>() {
                            object.data = object.data.pow(arg.data as u32);
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i32>>() {
                            object.data = object.data.pow(arg.data as u32);
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i16>>() {
                            object.data = object.data.pow(arg.data as u32);
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i8>>() {
                            object.data = object.data.pow(arg.data as u32);
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u64>>() {
                            object.data = object.data.pow(arg.data as u32);
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u32>>() {
                            object.data = object.data.pow(arg.data as u32);
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u16>>() {
                            object.data = object.data.pow(arg.data as u32);
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u8>>() {
                            object.data = object.data.pow(arg.data as u32);
                        } else if let Some(arg) = arg_mut.downcast_mut::<crate::object::primitive::PrimitiveObject<f64>>() {
                            arg.data = (object.data as f64).powf(arg.data);
                            drop(arg_mut);
                            context.pop();
                            return Ok(Some(original_arg));
                        } else if let Some(arg) = arg_mut.downcast_mut::<crate::object::primitive::PrimitiveObject<f32>>() {
                            arg.data = (object.data as f32).powf(arg.data);
                            drop(arg_mut);
                            context.pop();
                            return Ok(Some(original_arg));
                        } else {
                            return Err(Fault::InvalidType(format!("Number pow: Not a number")));
                        }
                    }
                    _ => {
                        return Err(Fault::InvalidType(format!("Number pow: expected {}", stringify!($type))));
                    }
                }
            }
            Ok(None)
        }

        fn $is_zero(object: ObjectBox, _: &mut crate::object::ContextData) -> Result<Option<ObjectBox>, Fault> {
            let object = object.borrow();
            if let Some(object) = object.downcast_ref::<crate::object::primitive::PrimitiveObject<$type>>() {
                if object.data.is_zero() {
                    return Ok(Some(crate::object::create_boolean(true)));
                } else {
                    return Ok(Some(crate::object::create_boolean(false)));
                }
            } else {
                return Err(Fault::InvalidType(format!("Number is_zero: expected {}", stringify!($type))))
            }
        }
    };
}

#[macro_export]
macro_rules! primitive_base_ops {
    ($type:ty, $equals:ident, $to_string:ident, $order:ident) => {
        fn $equals(object: ObjectBox, context: &mut crate::object::ContextData) -> Result<Option<ObjectBox>, Fault> {
            {
                let mut object = object.borrow_mut();
                let object = object.downcast_mut::<crate::object::primitive::PrimitiveObject<$type>>();
                let original_arg = context.arguments[0].clone();
                let mut arg_mut = original_arg.borrow_mut();
                match object {
                    Some(object) => {
                        if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i64>>() {
                            if object.data == arg.data as $type {
                                return Ok(Some(crate::object::create_boolean(true)));
                            } else {
                                return Ok(Some(crate::object::create_boolean(false)));
                            }
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i32>>() {
                            if object.data == arg.data as $type {
                                return Ok(Some(crate::object::create_boolean(true)));
                            } else {
                                return Ok(Some(crate::object::create_boolean(false)));
                            }
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i16>>() {
                            if object.data == arg.data as $type {
                                return Ok(Some(crate::object::create_boolean(true)));
                            } else {
                                return Ok(Some(crate::object::create_boolean(false)));
                            }
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i8>>() {
                            if object.data == arg.data as $type {
                                return Ok(Some(crate::object::create_boolean(true)));
                            } else {
                                return Ok(Some(crate::object::create_boolean(false)));
                            }
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u64>>() {
                            if object.data == arg.data as $type {
                                return Ok(Some(crate::object::create_boolean(true)));
                            } else {
                                return Ok(Some(crate::object::create_boolean(false)));
                            }
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u32>>() {
                            if object.data == arg.data as $type {
                                return Ok(Some(crate::object::create_boolean(true)));
                            } else {
                                return Ok(Some(crate::object::create_boolean(false)));
                            }
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u16>>() {
                            if object.data == arg.data as $type {
                                return Ok(Some(crate::object::create_boolean(true)));
                            } else {
                                return Ok(Some(crate::object::create_boolean(false)));
                            }
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u8>>() {
                            if object.data == arg.data as $type {
                                return Ok(Some(crate::object::create_boolean(true)));
                            } else {
                                return Ok(Some(crate::object::create_boolean(false)));
                            }
                        } else if let Some(arg) = arg_mut.downcast_mut::<crate::object::primitive::PrimitiveObject<f64>>() {
                            if object.data as f64 == arg.data as f64 {
                                return Ok(Some(crate::object::create_boolean(true)));
                            } else {
                                return Ok(Some(crate::object::create_boolean(false)));
                            }
                        } else if let Some(arg) = arg_mut.downcast_mut::<crate::object::primitive::PrimitiveObject<f32>>() {
                            if object.data as f32 == arg.data as f32 {
                                return Ok(Some(crate::object::create_boolean(true)));
                            } else {
                                return Ok(Some(crate::object::create_boolean(false)));
                            }
                        } else {
                            return Err(Fault::InvalidType(format!("Number equals: Not a number")));
                        }
                    }
                    _ => {
                        return Err(Fault::InvalidType(format!("Number equals: expected {}", stringify!($type))));
                    }
                }
            }
        }
        
        fn $to_string(object: ObjectBox, _: &mut crate::object::ContextData) -> Result<Option<ObjectBox>, Fault> {
            let object = object.borrow();
            if let Some(object) = object.downcast_ref::<crate::object::primitive::PrimitiveObject<$type>>() {
                let string = object.data.to_string();
                return Ok(Some(crate::object::create_string(string)));
            } else {
                return Err(Fault::InvalidType(format!("Number to_string: expected {}", stringify!($type))))
            }
        }


        fn $order(object: ObjectBox, context: &mut crate::object::ContextData) -> Result<Option<ObjectBox>, Fault> {
            {
                let mut object = object.borrow_mut();
                let object = object.downcast_mut::<crate::object::primitive::PrimitiveObject<$type>>();
                let original_arg = context.arguments[0].clone();
                let mut arg_mut = original_arg.borrow_mut();
                match object {
                    Some(object) => {
                        if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i64>>() {
                            if object.data < arg.data as $type {
                                return Ok(Some(crate::object::create_i8(-1)));
                            } else if object.data > arg.data as $type {
                                return Ok(Some(crate::object::create_i8(1)));
                            } else {
                                return Ok(Some(crate::object::create_i8(0)));
                            }
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i32>>() {
                            if object.data < arg.data as $type {
                                return Ok(Some(crate::object::create_i8(-1)));
                            } else if object.data > arg.data as $type {
                                return Ok(Some(crate::object::create_i8(1)));
                            } else {
                                return Ok(Some(crate::object::create_i8(0)));
                            }
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i16>>() {
                            if object.data < arg.data as $type {
                                return Ok(Some(crate::object::create_i8(-1)));
                            } else if object.data > arg.data as $type {
                                return Ok(Some(crate::object::create_i8(1)));
                            } else {
                                return Ok(Some(crate::object::create_i8(0)));
                            }
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<i8>>() {
                            if object.data < arg.data as $type {
                                return Ok(Some(crate::object::create_i8(-1)));
                            } else if object.data > arg.data as $type {
                                return Ok(Some(crate::object::create_i8(1)));
                            } else {
                                return Ok(Some(crate::object::create_i8(0)));
                            }
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u64>>() {
                            if object.data < arg.data as $type {
                                return Ok(Some(crate::object::create_i8(-1)));
                            } else if object.data > arg.data as $type {
                                return Ok(Some(crate::object::create_i8(1)));
                            } else {
                                return Ok(Some(crate::object::create_i8(0)));
                            }
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u32>>() {
                            if object.data < arg.data as $type {
                                return Ok(Some(crate::object::create_i8(-1)));
                            } else if object.data > arg.data as $type {
                                return Ok(Some(crate::object::create_i8(1)));
                            } else {
                                return Ok(Some(crate::object::create_i8(0)));
                            }
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u16>>() {
                            if object.data < arg.data as $type {
                                return Ok(Some(crate::object::create_i8(-1)));
                            } else if object.data > arg.data as $type {
                                return Ok(Some(crate::object::create_i8(1)));
                            } else {
                                return Ok(Some(crate::object::create_i8(0)));
                            }
                        } else if let Some(arg) = arg_mut.downcast_ref::<crate::object::primitive::PrimitiveObject<u8>>() {
                            if object.data < arg.data as $type {
                                return Ok(Some(crate::object::create_i8(-1)));
                            } else if object.data > arg.data as $type {
                                return Ok(Some(crate::object::create_i8(1)));
                            } else {
                                return Ok(Some(crate::object::create_i8(0)));
                            }
                        } else if let Some(arg) = arg_mut.downcast_mut::<crate::object::primitive::PrimitiveObject<f64>>() {
                            if (object.data as f64) < arg.data {
                                return Ok(Some(crate::object::create_i8(-1)));
                            } else if object.data > arg.data as $type {
                                return Ok(Some(crate::object::create_i8(1)));
                            } else {
                                return Ok(Some(crate::object::create_i8(0)));
                            }
                        } else if let Some(arg) = arg_mut.downcast_mut::<crate::object::primitive::PrimitiveObject<f32>>() {
                            if (object.data as f32) < arg.data {
                                return Ok(Some(crate::object::create_i8(-1)));
                            } else if object.data > arg.data as $type {
                                return Ok(Some(crate::object::create_i8(1)));
                            } else {
                                return Ok(Some(crate::object::create_i8(0)));
                            }
                        } else {
                            return Err(Fault::InvalidType(format!("Number order: Not a number")));
                        }
                    }
                    _ => {
                        return Err(Fault::InvalidType(format!("Number order: expected {}", stringify!($type))));
                    }
                }
            }
        }
    };
}
