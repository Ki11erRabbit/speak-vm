use super::{Method, ObjectBox, VTable};
use crate::object::Object;
use super::Fault;
use std::collections::HashMap;
use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;
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
        Rc::new(RefCell::new(NumberObject {super_object: Some(parent), vtable: VTable::new_empty()})) as ObjectBox
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
        Rc::new(RefCell::new(number))
    }
    fn initialize(&mut self, _: Vec<ObjectBox>, vtable: VTable) {
        self.vtable.extend(NumberObject::make_vtable());
        self.vtable.extend(vtable);
    }
}

fn number_add(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn number_subtract(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn number_multiply(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn number_divide(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn number_modulo(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn number_abs(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn number_pow(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn number_is_zero(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}


#[macro_export]
macro_rules! create_type_ops {
    ($type:ty, $add_name:ident, $sub_name:ident, $mul_name:ident, $div_name:ident, $mod_name:ident) => {

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
