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
use num_integer::Integer;



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
    fn duplicate(&self) -> ObjectBox<dyn Object> {
        IntegerObject::make_object(self.class.clone(), self.super_object.clone().unwrap())
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

macro_rules! create_integer_ops {
    ($type:ty, $divides:ident, $shr:ident, $shl:ident, $and:ident, $or:ident, $xor:ident) => {
        fn $divides(object: ObjectBox<dyn Object>, context: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            let object = object.borrow();
            let object = object.downcast_ref::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            let other = context.arguments[0].borrow();
            if let Some(other) = other.downcast_ref::<PrimitiveObject<i64>>() {
                if object.data.is_multiple_of(&(other.data as $type)) {
                    Ok(Some(context.create_boolean(true)))
                } else {
                    Ok(Some(context.create_boolean(false)))
                }
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u64>>() {
                if object.data.is_multiple_of(&(other.data as $type)) {
                    Ok(Some(context.create_boolean(true)))
                } else {
                    Ok(Some(context.create_boolean(false)))
                }
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<i32>>() {
                if object.data.is_multiple_of(&(other.data as $type)) {
                    Ok(Some(context.create_boolean(true)))
                } else {
                    Ok(Some(context.create_boolean(false)))
                }
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u32>>() {
                if object.data.is_multiple_of(&(other.data as $type)) {
                    Ok(Some(context.create_boolean(true)))
                } else {
                    Ok(Some(context.create_boolean(false)))
                }
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<i16>>() {
                if object.data.is_multiple_of(&(other.data as $type)) {
                    Ok(Some(context.create_boolean(true)))
                } else {
                    Ok(Some(context.create_boolean(false)))
                }
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u16>>() {
                if object.data.is_multiple_of(&(other.data as $type)) {
                    Ok(Some(context.create_boolean(true)))
                } else {
                    Ok(Some(context.create_boolean(false)))
                }
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<i8>>() {
                if object.data.is_multiple_of(&(other.data as $type)) {
                    Ok(Some(context.create_boolean(true)))
                } else {
                    Ok(Some(context.create_boolean(false)))
                }
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u8>>() {
                if object.data.is_multiple_of(&(other.data as $type)) {
                    Ok(Some(context.create_boolean(true)))
                } else {
                    Ok(Some(context.create_boolean(false)))
                }
            } else {
                Err(Fault::InvalidType)
            }
        }

        fn $shr(object: ObjectBox<dyn Object>, context: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            let mut object = object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            let other = context.arguments[0].borrow();
            if let Some(other) = other.downcast_ref::<PrimitiveObject<i64>>() {
                object.data >>= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u64>>() {
                object.data >>= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<i32>>() {
                object.data >>= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u32>>() {
                object.data >>= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<i16>>() {
                object.data >>= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u16>>() {
                object.data >>= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<i8>>() {
                object.data >>= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u8>>() {
                object.data >>= other.data as $type
            } else {
                return Err(Fault::InvalidType)
            }
            Ok(None)
        }

        fn $shl(object: ObjectBox<dyn Object>, context: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            let mut object = object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            let other = context.arguments[0].borrow();
            if let Some(other) = other.downcast_ref::<PrimitiveObject<i64>>() {
                object.data <<= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u64>>() {
                object.data <<= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<i32>>() {
                object.data <<= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u32>>() {
                object.data <<= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<i16>>() {
                object.data <<= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u16>>() {
                object.data <<= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<i8>>() {
                object.data <<= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u8>>() {
                object.data <<= other.data as $type
            } else {
                return Err(Fault::InvalidType)
            }
            Ok(None)
        }

        fn $and(object: ObjectBox<dyn Object>, context: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            let mut object = object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            let other = context.arguments[0].borrow();
            if let Some(other) = other.downcast_ref::<PrimitiveObject<i64>>() {
                object.data &= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u64>>() {
                object.data &= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<i32>>() {
                object.data &= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u32>>() {
                object.data &= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<i16>>() {
                object.data &= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u16>>() {
                object.data &= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<i8>>() {
                object.data &= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u8>>() {
                object.data &= other.data as $type
            } else {
                return Err(Fault::InvalidType)
            }
            Ok(None)
        }

        fn $or(object: ObjectBox<dyn Object>, context: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            let mut object = object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            let other = context.arguments[0].borrow();
            if let Some(other) = other.downcast_ref::<PrimitiveObject<i64>>() {
                object.data |= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u64>>() {
                object.data |= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<i32>>() {
                object.data |= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u32>>() {
                object.data |= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<i16>>() {
                object.data |= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u16>>() {
                object.data |= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<i8>>() {
                object.data |= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u8>>() {
                object.data |= other.data as $type
            } else {
                return Err(Fault::InvalidType)
            }
            Ok(None)
        }

        fn $xor(object: ObjectBox<dyn Object>, context: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            let mut object = object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            let other = context.arguments[0].borrow();
            if let Some(other) = other.downcast_ref::<PrimitiveObject<i64>>() {
                object.data ^= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u64>>() {
                object.data ^= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<i32>>() {
                object.data ^= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u32>>() {
                object.data ^= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<i16>>() {
                object.data ^= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u16>>() {
                object.data ^= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<i8>>() {
                object.data ^= other.data as $type
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u8>>() {
                object.data ^= other.data as $type
            } else {
                return Err(Fault::InvalidType)
            }
            Ok(None)
        }
        
    };

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
        methods.insert(String::from("divides"), Arc::new(Method::RustMethod { fun: Box::new(i64_divides) }));
        methods.insert(String::from("shift_right"), Arc::new(Method::RustMethod { fun: Box::new(i64_shr) }));
        methods.insert(String::from("shift_left"), Arc::new(Method::RustMethod { fun: Box::new(i64_shl) }));
        methods.insert(String::from("and"), Arc::new(Method::RustMethod { fun: Box::new(i64_and) }));
        methods.insert(String::from("or"), Arc::new(Method::RustMethod { fun: Box::new(i64_or) }));
        methods.insert(String::from("xor"), Arc::new(Method::RustMethod { fun: Box::new(i64_xor) }));

        
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
create_integer_ops!(i64, i64_divides, i64_shr, i64_shl, i64_and, i64_or, i64_xor);

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
        methods.insert(String::from("divides"), Arc::new(Method::RustMethod { fun: Box::new(u64_divides) }));
        methods.insert(String::from("shift_right"), Arc::new(Method::RustMethod { fun: Box::new(u64_shr) }));
        methods.insert(String::from("shift_left"), Arc::new(Method::RustMethod { fun: Box::new(u64_shl) }));
        methods.insert(String::from("and"), Arc::new(Method::RustMethod { fun: Box::new(u64_and) }));
        methods.insert(String::from("or"), Arc::new(Method::RustMethod { fun: Box::new(u64_or) }));
        methods.insert(String::from("xor"), Arc::new(Method::RustMethod { fun: Box::new(u64_xor) }));
        
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
create_integer_ops!(u64, u64_divides, u64_shr, u64_shl, u64_and, u64_or, u64_xor);

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
        methods.insert(String::from("divides"), Arc::new(Method::RustMethod { fun: Box::new(i32_divides) }));
        methods.insert(String::from("shift_right"), Arc::new(Method::RustMethod { fun: Box::new(i32_shr) }));
        methods.insert(String::from("shift_left"), Arc::new(Method::RustMethod { fun: Box::new(i32_shl) }));
        methods.insert(String::from("and"), Arc::new(Method::RustMethod { fun: Box::new(i32_and) }));
        methods.insert(String::from("or"), Arc::new(Method::RustMethod { fun: Box::new(i32_or) }));
        methods.insert(String::from("xor"), Arc::new(Method::RustMethod { fun: Box::new(i32_xor) }));

        
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
create_integer_ops!(i32, i32_divides, i32_shr, i32_shl, i32_and, i32_or, i32_xor);

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
        methods.insert(String::from("divides"), Arc::new(Method::RustMethod { fun: Box::new(u32_divides) }));
        methods.insert(String::from("shift_right"), Arc::new(Method::RustMethod { fun: Box::new(u32_shr) }));
        methods.insert(String::from("shift_left"), Arc::new(Method::RustMethod { fun: Box::new(u32_shl) }));
        methods.insert(String::from("and"), Arc::new(Method::RustMethod { fun: Box::new(u32_and) }));
        methods.insert(String::from("or"), Arc::new(Method::RustMethod { fun: Box::new(u32_or) }));
        methods.insert(String::from("xor"), Arc::new(Method::RustMethod { fun: Box::new(u32_xor) }));

        
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
create_integer_ops!(u32, u32_divides, u32_shr, u32_shl, u32_and, u32_or, u32_xor);

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
        methods.insert(String::from("divides"), Arc::new(Method::RustMethod { fun: Box::new(i16_divides) }));
        methods.insert(String::from("shift_right"), Arc::new(Method::RustMethod { fun: Box::new(i16_shr) }));
        methods.insert(String::from("shift_left"), Arc::new(Method::RustMethod { fun: Box::new(i16_shl) }));
        methods.insert(String::from("and"), Arc::new(Method::RustMethod { fun: Box::new(i16_and) }));
        methods.insert(String::from("or"), Arc::new(Method::RustMethod { fun: Box::new(i16_or) }));
        methods.insert(String::from("xor"), Arc::new(Method::RustMethod { fun: Box::new(i16_xor) }));

        
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
create_integer_ops!(i16, i16_divides, i16_shr, i16_shl, i16_and, i16_or, i16_xor);


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
        methods.insert(String::from("divides"), Arc::new(Method::RustMethod { fun: Box::new(u16_divides) }));
        methods.insert(String::from("shift_right"), Arc::new(Method::RustMethod { fun: Box::new(u16_shr) }));
        methods.insert(String::from("shift_left"), Arc::new(Method::RustMethod { fun: Box::new(u16_shl) }));
        methods.insert(String::from("and"), Arc::new(Method::RustMethod { fun: Box::new(u16_and) }));
        methods.insert(String::from("or"), Arc::new(Method::RustMethod { fun: Box::new(u16_or) }));
        methods.insert(String::from("xor"), Arc::new(Method::RustMethod { fun: Box::new(u16_xor) }));

        
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
create_integer_ops!(u16, u16_divides, u16_shr, u16_shl, u16_and, u16_or, u16_xor);

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
        methods.insert(String::from("divides"), Arc::new(Method::RustMethod { fun: Box::new(i8_divides) }));
        methods.insert(String::from("shift_right"), Arc::new(Method::RustMethod { fun: Box::new(i8_shr) }));
        methods.insert(String::from("shift_left"), Arc::new(Method::RustMethod { fun: Box::new(i8_shl) }));
        methods.insert(String::from("and"), Arc::new(Method::RustMethod { fun: Box::new(i8_and) }));
        methods.insert(String::from("or"), Arc::new(Method::RustMethod { fun: Box::new(i8_or) }));
        methods.insert(String::from("xor"), Arc::new(Method::RustMethod { fun: Box::new(i8_xor) }));

        
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
create_integer_ops!(i8, i8_divides, i8_shr, i8_shl, i8_and, i8_or, i8_xor);

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
        methods.insert(String::from("divides"), Arc::new(Method::RustMethod { fun: Box::new(u8_divides) }));
        methods.insert(String::from("shift_right"), Arc::new(Method::RustMethod { fun: Box::new(u8_shr) }));
        methods.insert(String::from("shift_left"), Arc::new(Method::RustMethod { fun: Box::new(u8_shl) }));
        methods.insert(String::from("and"), Arc::new(Method::RustMethod { fun: Box::new(u8_and) }));
        methods.insert(String::from("or"), Arc::new(Method::RustMethod { fun: Box::new(u8_or) }));
        methods.insert(String::from("xor"), Arc::new(Method::RustMethod { fun: Box::new(u8_xor) }));

        
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
create_integer_ops!(u8, u8_divides, u8_shr, u8_shl, u8_and, u8_or, u8_xor);


