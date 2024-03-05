use super::{Class, Method, ObjectBox};
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};
use crate::object::Object;
use crate::object::ContextData;
use std::sync::Arc;
use super::Fault;
use num_traits::Zero;
use crate::create_type_ops;
use crate::object::primitive::PrimitiveObject;
use num_integer::Integer;
use crate::object::create_boolean;
use crate::object::VTable;


pub struct IntegerObject {
    class: Arc<Class>,
    super_object: Option<ObjectBox>,
    vtable: VTable,
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
                       parent: ObjectBox) -> ObjectBox {
        let out = Rc::new(RefCell::new(IntegerObject{class, super_object: Some(parent), vtable: VTable::new_empty()}));
        return out as ObjectBox;
    }
}


impl Object for IntegerObject {
    fn get_class(&self) -> Arc<Class> {
        self.class.clone()
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        self.super_object.clone()
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox> {
        panic!("Integer objects do not have fields")
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox) {
        panic!("Integer objects do not have fields")
    }
    fn size(&self) -> Option<usize> {
        None
    }
    fn duplicate(&self) -> ObjectBox {
        IntegerObject::make_object(self.class.clone(), self.super_object.clone().unwrap())
    }
    fn initialize(&mut self, arguments: Vec<ObjectBox>, vtable: crate::object::VTable) {
        
    }
}

fn integer_divides(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn integer_shift_right(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn integer_shift_left(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn integer_bitwise_and(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn integer_bitwise_or(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn integer_bitwise_xor(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

macro_rules! create_integer_ops {
    ($type:ty, $divides:ident, $shr:ident, $shl:ident, $and:ident, $or:ident, $xor:ident) => {
        fn $divides(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
            let object = object.borrow();
            let object = object.downcast_ref::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            let other = context.arguments[0].borrow();
            if let Some(other) = other.downcast_ref::<PrimitiveObject<i64>>() {
                if object.data.is_multiple_of(&(other.data as $type)) {
                    Ok(Some(create_boolean(true)))
                } else {
                    Ok(Some(create_boolean(false)))
                }
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u64>>() {
                if object.data.is_multiple_of(&(other.data as $type)) {
                    Ok(Some(create_boolean(true)))
                } else {
                    Ok(Some(create_boolean(false)))
                }
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<i32>>() {
                if object.data.is_multiple_of(&(other.data as $type)) {
                    Ok(Some(create_boolean(true)))
                } else {
                    Ok(Some(create_boolean(false)))
                }
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u32>>() {
                if object.data.is_multiple_of(&(other.data as $type)) {
                    Ok(Some(create_boolean(true)))
                } else {
                    Ok(Some(create_boolean(false)))
                }
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<i16>>() {
                if object.data.is_multiple_of(&(other.data as $type)) {
                    Ok(Some(create_boolean(true)))
                } else {
                    Ok(Some(create_boolean(false)))
                }
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u16>>() {
                if object.data.is_multiple_of(&(other.data as $type)) {
                    Ok(Some(create_boolean(true)))
                } else {
                    Ok(Some(create_boolean(false)))
                }
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<i8>>() {
                if object.data.is_multiple_of(&(other.data as $type)) {
                    Ok(Some(create_boolean(true)))
                } else {
                    Ok(Some(create_boolean(false)))
                }
            } else if let Some(other) = other.downcast_ref::<PrimitiveObject<u8>>() {
                if object.data.is_multiple_of(&(other.data as $type)) {
                    Ok(Some(create_boolean(true)))
                } else {
                    Ok(Some(create_boolean(false)))
                }
            } else {
                Err(Fault::InvalidType)
            }
        }

        fn $shr(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
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

        fn $shl(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
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

        fn $and(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
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

        fn $or(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
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

        fn $xor(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
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
        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Arc<Class>,
                           parent: ObjectBox,
                           data: i64) -> ObjectBox {
        let out = Rc::new(RefCell::new(PrimitiveObject::new(class, Some(parent), data)));
        return out as ObjectBox;
    }
}


impl Object for PrimitiveObject<i64> {
    fn get_class(&self) -> Arc<Class> {
        self.class.clone()
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        self.super_object.clone()
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox> {
        panic!("Integer objects do not have fields")
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox) {
        panic!("Integer objects do not have fields")
    }
    fn size(&self) -> Option<usize> {
        None
    }
    fn duplicate(&self) -> ObjectBox {
        I64Object::make_object(self.class.clone(), self.super_object.clone().unwrap(), self.data)
    }
    fn initialize(&mut self, _: Vec<ObjectBox>, vtable: VTable) {
        let mut number_vtable = HashMap::new();
        number_vtable.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(i64_add) }));
        number_vtable.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(i64_sub) }));
        number_vtable.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(i64_mul) }));
        number_vtable.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(i64_div) }));
        number_vtable.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(i64_mod) }));
        let number_vtable = VTable::new(number_vtable);

        let mut integer_vtable = HashMap::new();
        integer_vtable.insert(String::from("divides"), Arc::new(Method::RustMethod { fun: Box::new(i64_divides) }));
        integer_vtable.insert(String::from("shift_right"), Arc::new(Method::RustMethod { fun: Box::new(i64_shr) }));
        integer_vtable.insert(String::from("shift_left"), Arc::new(Method::RustMethod { fun: Box::new(i64_shl) }));
        integer_vtable.insert(String::from("and"), Arc::new(Method::RustMethod { fun: Box::new(i64_and) }));
        integer_vtable.insert(String::from("or"), Arc::new(Method::RustMethod { fun: Box::new(i64_or) }));
        integer_vtable.insert(String::from("xor"), Arc::new(Method::RustMethod { fun: Box::new(i64_xor) }));
        let integer_vtable = VTable::new(integer_vtable);

        let integer_object = self.get_super_object().unwrap().clone();
        let mut integer_object = integer_object.borrow_mut();
        integer_object.initialize(Vec::new(), integer_vtable);
        let number_object = integer_object.get_super_object().unwrap().clone();
        let mut number_object = number_object.borrow_mut();
        number_object.initialize(Vec::new(), number_vtable);
    }
}

create_type_ops!(i64, i64_add, i64_sub, i64_mul, i64_div, i64_mod);
create_integer_ops!(i64, i64_divides, i64_shr, i64_shl, i64_and, i64_or, i64_xor);

pub struct U64Object {
}

impl U64Object {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let mut methods = HashMap::new();
        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Arc<Class>,
                           parent: ObjectBox,
                           data: u64) -> ObjectBox {
        let out = Rc::new(RefCell::new(PrimitiveObject::new(class, Some(parent), data)));
        return out as ObjectBox;
    }
}

impl Object for PrimitiveObject<u64> {
    fn get_class(&self) -> Arc<Class> {
        self.class.clone()
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        self.super_object.clone()
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox> {
        panic!("Integer objects do not have fields")
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox) {
        panic!("Integer objects do not have fields")
    }
    fn size(&self) -> Option<usize> {
        None
    }
    fn duplicate(&self) -> ObjectBox {
        U64Object::make_object(self.class.clone(), self.super_object.clone().unwrap(), self.data)
    }
    fn initialize(&mut self, _: Vec<ObjectBox>, vtable: VTable) {
        let mut number_vtable = HashMap::new();
        number_vtable.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(u64_add) }));
        number_vtable.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(u64_sub) }));
        number_vtable.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(u64_mul) }));
        number_vtable.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(u64_div) }));
        number_vtable.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(u64_mod) }));
        let number_vtable = VTable::new(number_vtable);

        let mut integer_vtable = HashMap::new();
        integer_vtable.insert(String::from("divides"), Arc::new(Method::RustMethod { fun: Box::new(u64_divides) }));
        integer_vtable.insert(String::from("shift_right"), Arc::new(Method::RustMethod { fun: Box::new(u64_shr) }));
        integer_vtable.insert(String::from("shift_left"), Arc::new(Method::RustMethod { fun: Box::new(u64_shl) }));
        integer_vtable.insert(String::from("and"), Arc::new(Method::RustMethod { fun: Box::new(u64_and) }));
        integer_vtable.insert(String::from("or"), Arc::new(Method::RustMethod { fun: Box::new(u64_or) }));
        integer_vtable.insert(String::from("xor"), Arc::new(Method::RustMethod { fun: Box::new(u64_xor) }));
        let integer_vtable = VTable::new(integer_vtable);

        let integer_object = self.get_super_object().unwrap().clone();
        let mut integer_object = integer_object.borrow_mut();
        integer_object.initialize(Vec::new(), integer_vtable);
        let number_object = integer_object.get_super_object().unwrap().clone();
        let mut number_object = number_object.borrow_mut();
        number_object.initialize(Vec::new(), number_vtable);
    }
}

create_type_ops!(u64, u64_add, u64_sub, u64_mul, u64_div, u64_mod);
create_integer_ops!(u64, u64_divides, u64_shr, u64_shl, u64_and, u64_or, u64_xor);

pub struct I32Object {
}

impl I32Object {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let mut methods = HashMap::new();

        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Arc<Class>,
                           parent: ObjectBox,
                           data: i32) -> ObjectBox {
        let out = Rc::new(RefCell::new(PrimitiveObject::new(class, Some(parent), data)));
        return out as ObjectBox;
    }
}

impl Object for PrimitiveObject<i32> {
    fn get_class(&self) -> Arc<Class> {
        self.class.clone()
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        self.super_object.clone()
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox> {
        panic!("Integer objects do not have fields")
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox) {
        panic!("Integer objects do not have fields")
    }
    fn size(&self) -> Option<usize> {
        None
    }
    fn duplicate(&self) -> ObjectBox {
        I32Object::make_object(self.class.clone(), self.super_object.clone().unwrap(), self.data)
    }
    fn initialize(&mut self, _: Vec<ObjectBox>, vtable: VTable) {
        let mut number_vtable = HashMap::new();
        number_vtable.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(i32_add) }));
        number_vtable.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(i32_sub) }));
        number_vtable.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(i32_mul) }));
        number_vtable.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(i32_div) }));
        number_vtable.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(i32_mod) }));
        let number_vtable = VTable::new(number_vtable);

        let mut integer_vtable = HashMap::new();
        integer_vtable.insert(String::from("divides"), Arc::new(Method::RustMethod { fun: Box::new(i32_divides) }));
        integer_vtable.insert(String::from("shift_right"), Arc::new(Method::RustMethod { fun: Box::new(i32_shr) }));
        integer_vtable.insert(String::from("shift_left"), Arc::new(Method::RustMethod { fun: Box::new(i32_shl) }));
        integer_vtable.insert(String::from("and"), Arc::new(Method::RustMethod { fun: Box::new(i32_and) }));
        integer_vtable.insert(String::from("or"), Arc::new(Method::RustMethod { fun: Box::new(i32_or) }));
        integer_vtable.insert(String::from("xor"), Arc::new(Method::RustMethod { fun: Box::new(i32_xor) }));
        let integer_vtable = VTable::new(integer_vtable);

        let integer_object = self.get_super_object().unwrap().clone();
        let mut integer_object = integer_object.borrow_mut();
        integer_object.initialize(Vec::new(), integer_vtable);
        let number_object = integer_object.get_super_object().unwrap().clone();
        let mut number_object = number_object.borrow_mut();
        number_object.initialize(Vec::new(), number_vtable);
    }
}

create_type_ops!(i32, i32_add, i32_sub, i32_mul, i32_div, i32_mod);
create_integer_ops!(i32, i32_divides, i32_shr, i32_shl, i32_and, i32_or, i32_xor);

pub struct U32Object {
}

impl U32Object {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let mut methods = HashMap::new();

        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Arc<Class>,
                           parent: ObjectBox,
                           data: u32) -> ObjectBox {
        let out = Rc::new(RefCell::new(PrimitiveObject::new(class, Some(parent), data)));
        return out as ObjectBox;
    }
}


impl Object for PrimitiveObject<u32> {
    fn get_class(&self) -> Arc<Class> {
        self.class.clone()
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        self.super_object.clone()
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox> {
        panic!("Integer objects do not have fields")
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox) {
        panic!("Integer objects do not have fields")
    }
    fn size(&self) -> Option<usize> {
        None
    }
    fn duplicate(&self) -> ObjectBox {
        U32Object::make_object(self.class.clone(), self.super_object.clone().unwrap(), self.data)
    }
    fn initialize(&mut self, _: Vec<ObjectBox>, vtable: VTable) {
        let mut number_vtable = HashMap::new();
        number_vtable.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(u32_add) }));
        number_vtable.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(u32_sub) }));
        number_vtable.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(u32_mul) }));
        number_vtable.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(u32_div) }));
        number_vtable.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(u32_mod) }));
        let number_vtable = VTable::new(number_vtable);

        let mut integer_vtable = HashMap::new();
        integer_vtable.insert(String::from("divides"), Arc::new(Method::RustMethod { fun: Box::new(u32_divides) }));
        integer_vtable.insert(String::from("shift_right"), Arc::new(Method::RustMethod { fun: Box::new(u32_shr) }));
        integer_vtable.insert(String::from("shift_left"), Arc::new(Method::RustMethod { fun: Box::new(u32_shl) }));
        integer_vtable.insert(String::from("and"), Arc::new(Method::RustMethod { fun: Box::new(u32_and) }));
        integer_vtable.insert(String::from("or"), Arc::new(Method::RustMethod { fun: Box::new(u32_or) }));
        integer_vtable.insert(String::from("xor"), Arc::new(Method::RustMethod { fun: Box::new(u32_xor) }));
        let integer_vtable = VTable::new(integer_vtable);

        let integer_object = self.get_super_object().unwrap().clone();
        let mut integer_object = integer_object.borrow_mut();
        integer_object.initialize(Vec::new(), integer_vtable);
        let number_object = integer_object.get_super_object().unwrap().clone();
        let mut number_object = number_object.borrow_mut();
        number_object.initialize(Vec::new(), number_vtable);
    }
}

create_type_ops!(u32, u32_add, u32_sub, u32_mul, u32_div, u32_mod);
create_integer_ops!(u32, u32_divides, u32_shr, u32_shl, u32_and, u32_or, u32_xor);

pub struct I16Object {
}

impl I16Object {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let mut methods = HashMap::new();
        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Arc<Class>,
                           parent: ObjectBox,
                           data: i16) -> ObjectBox {
        let out = Rc::new(RefCell::new(PrimitiveObject::new(class, Some(parent), data)));
        return out as ObjectBox;
    }
}

impl Object for PrimitiveObject<i16> {
    fn get_class(&self) -> Arc<Class> {
        self.class.clone()
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        self.super_object.clone()
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox> {
        panic!("Integer objects do not have fields")
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox) {
        panic!("Integer objects do not have fields")
    }
    fn size(&self) -> Option<usize> {
        None
    }
    fn duplicate(&self) -> ObjectBox {
        I16Object::make_object(self.class.clone(), self.super_object.clone().unwrap(), self.data)
    }
    fn initialize(&mut self, _: Vec<ObjectBox>, vtable: VTable) {
        let mut number_vtable = HashMap::new();
        number_vtable.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(i16_add) }));
        number_vtable.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(i16_sub) }));
        number_vtable.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(i16_mul) }));
        number_vtable.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(i16_div) }));
        number_vtable.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(i16_mod) }));
        let number_vtable = VTable::new(number_vtable);

        let mut integer_vtable = HashMap::new();
        integer_vtable.insert(String::from("divides"), Arc::new(Method::RustMethod { fun: Box::new(i16_divides) }));
        integer_vtable.insert(String::from("shift_right"), Arc::new(Method::RustMethod { fun: Box::new(i16_shr) }));
        integer_vtable.insert(String::from("shift_left"), Arc::new(Method::RustMethod { fun: Box::new(i16_shl) }));
        integer_vtable.insert(String::from("and"), Arc::new(Method::RustMethod { fun: Box::new(i16_and) }));
        integer_vtable.insert(String::from("or"), Arc::new(Method::RustMethod { fun: Box::new(i16_or) }));
        integer_vtable.insert(String::from("xor"), Arc::new(Method::RustMethod { fun: Box::new(i16_xor) }));
        let integer_vtable = VTable::new(integer_vtable);

        let integer_object = self.get_super_object().unwrap().clone();
        let mut integer_object = integer_object.borrow_mut();
        integer_object.initialize(Vec::new(), integer_vtable);
        let number_object = integer_object.get_super_object().unwrap().clone();
        let mut number_object = number_object.borrow_mut();
        number_object.initialize(Vec::new(), number_vtable);
    }
}

create_type_ops!(i16, i16_add, i16_sub, i16_mul, i16_div, i16_mod);
create_integer_ops!(i16, i16_divides, i16_shr, i16_shl, i16_and, i16_or, i16_xor);


pub struct U16Object {
}

impl U16Object {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let mut methods = HashMap::new();

        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Arc<Class>,
                           parent: ObjectBox,
                           data: u16) -> ObjectBox {
        let out = Rc::new(RefCell::new(PrimitiveObject::new(class, Some(parent), data)));
        return out as ObjectBox;
    }
}

impl Object for PrimitiveObject<u16> {
    fn get_class(&self) -> Arc<Class> {
        self.class.clone()
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        self.super_object.clone()
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox> {
        panic!("Integer objects do not have fields")
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox) {
        panic!("Integer objects do not have fields")
    }
    fn size(&self) -> Option<usize> {
        None
    }
    fn duplicate(&self) -> ObjectBox {
        U16Object::make_object(self.class.clone(), self.super_object.clone().unwrap(), self.data)
    }
    fn initialize(&mut self, _: Vec<ObjectBox>, vtable: VTable) {
        let mut number_vtable = HashMap::new();
        number_vtable.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(u16_add) }));
        number_vtable.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(u16_sub) }));
        number_vtable.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(u16_mul) }));
        number_vtable.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(u16_div) }));
        number_vtable.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(u16_mod) }));
        let number_vtable = VTable::new(number_vtable);

        let mut integer_vtable = HashMap::new();
        integer_vtable.insert(String::from("divides"), Arc::new(Method::RustMethod { fun: Box::new(u16_divides) }));
        integer_vtable.insert(String::from("shift_right"), Arc::new(Method::RustMethod { fun: Box::new(u16_shr) }));
        integer_vtable.insert(String::from("shift_left"), Arc::new(Method::RustMethod { fun: Box::new(u16_shl) }));
        integer_vtable.insert(String::from("and"), Arc::new(Method::RustMethod { fun: Box::new(u16_and) }));
        integer_vtable.insert(String::from("or"), Arc::new(Method::RustMethod { fun: Box::new(u16_or) }));
        integer_vtable.insert(String::from("xor"), Arc::new(Method::RustMethod { fun: Box::new(u16_xor) }));
        let integer_vtable = VTable::new(integer_vtable);

        let integer_object = self.get_super_object().unwrap().clone();
        let mut integer_object = integer_object.borrow_mut();
        integer_object.initialize(Vec::new(), integer_vtable);
        let number_object = integer_object.get_super_object().unwrap().clone();
        let mut number_object = number_object.borrow_mut();
        number_object.initialize(Vec::new(), number_vtable);
    }
}

create_type_ops!(u16, u16_add, u16_sub, u16_mul, u16_div, u16_mod);
create_integer_ops!(u16, u16_divides, u16_shr, u16_shl, u16_and, u16_or, u16_xor);

pub struct I8Object {
}

impl I8Object {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let mut methods = HashMap::new();

        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Arc<Class>,
                           parent: ObjectBox,
                           data: i8) -> ObjectBox {
        let out = Rc::new(RefCell::new(PrimitiveObject::new(class, Some(parent), data)));
        return out as ObjectBox;
    }
}

impl Object for PrimitiveObject<i8> {
    fn get_class(&self) -> Arc<Class> {
        self.class.clone()
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        self.super_object.clone()
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox> {
        panic!("Integer objects do not have fields")
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox) {
        panic!("Integer objects do not have fields")
    }
    fn size(&self) -> Option<usize> {
        None
    }
    fn duplicate(&self) -> ObjectBox {
        I8Object::make_object(self.class.clone(), self.super_object.clone().unwrap(), self.data)
    }
    fn initialize(&mut self, _: Vec<ObjectBox>, vtable: VTable) {
        let mut number_vtable = HashMap::new();
        number_vtable.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(i8_add) }));
        number_vtable.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(i8_sub) }));
        number_vtable.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(i8_mul) }));
        number_vtable.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(i8_div) }));
        number_vtable.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(i8_mod) }));
        let number_vtable = VTable::new(number_vtable);

        let mut integer_vtable = HashMap::new();
        integer_vtable.insert(String::from("divides"), Arc::new(Method::RustMethod { fun: Box::new(i8_divides) }));
        integer_vtable.insert(String::from("shift_right"), Arc::new(Method::RustMethod { fun: Box::new(i8_shr) }));
        integer_vtable.insert(String::from("shift_left"), Arc::new(Method::RustMethod { fun: Box::new(i8_shl) }));
        integer_vtable.insert(String::from("and"), Arc::new(Method::RustMethod { fun: Box::new(i8_and) }));
        integer_vtable.insert(String::from("or"), Arc::new(Method::RustMethod { fun: Box::new(i8_or) }));
        integer_vtable.insert(String::from("xor"), Arc::new(Method::RustMethod { fun: Box::new(i8_xor) }));
        let integer_vtable = VTable::new(integer_vtable);

        let integer_object = self.get_super_object().unwrap().clone();
        let mut integer_object = integer_object.borrow_mut();
        integer_object.initialize(Vec::new(), integer_vtable);
        let number_object = integer_object.get_super_object().unwrap().clone();
        let mut number_object = number_object.borrow_mut();
        number_object.initialize(Vec::new(), number_vtable);
    }
}

create_type_ops!(i8, i8_add, i8_sub, i8_mul, i8_div, i8_mod);
create_integer_ops!(i8, i8_divides, i8_shr, i8_shl, i8_and, i8_or, i8_xor);

pub struct U8Object {
}

impl U8Object {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let mut methods = HashMap::new();

        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Arc<Class>,
                           parent: ObjectBox,
                           data: u8) -> ObjectBox {
        let out = Rc::new(RefCell::new(PrimitiveObject::new(class, Some(parent), data)));
        return out as ObjectBox;
    }
}

impl Object for PrimitiveObject<u8> {
    fn get_class(&self) -> Arc<Class> {
        self.class.clone()
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        self.super_object.clone()
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox> {
        panic!("Integer objects do not have fields")
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox) {
        panic!("Integer objects do not have fields")
    }
    fn size(&self) -> Option<usize> {
        None
    }
    fn duplicate(&self) -> ObjectBox {
        U8Object::make_object(self.class.clone(), self.super_object.clone().unwrap(), self.data)
    }
    fn initialize(&mut self, _: Vec<ObjectBox>, vtable: VTable) {
        let mut number_vtable = HashMap::new();
        number_vtable.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(u8_add) }));
        number_vtable.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(u8_sub) }));
        number_vtable.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(u8_mul) }));
        number_vtable.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(u8_div) }));
        number_vtable.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(u8_mod) }));
        let number_vtable = VTable::new(number_vtable);

        let mut integer_vtable = HashMap::new();
        integer_vtable.insert(String::from("divides"), Arc::new(Method::RustMethod { fun: Box::new(u8_divides) }));
        integer_vtable.insert(String::from("shift_right"), Arc::new(Method::RustMethod { fun: Box::new(u8_shr) }));
        integer_vtable.insert(String::from("shift_left"), Arc::new(Method::RustMethod { fun: Box::new(u8_shl) }));
        integer_vtable.insert(String::from("and"), Arc::new(Method::RustMethod { fun: Box::new(u8_and) }));
        integer_vtable.insert(String::from("or"), Arc::new(Method::RustMethod { fun: Box::new(u8_or) }));
        integer_vtable.insert(String::from("xor"), Arc::new(Method::RustMethod { fun: Box::new(u8_xor) }));
        let integer_vtable = VTable::new(integer_vtable);

        let integer_object = self.get_super_object().unwrap().clone();
        let mut integer_object = integer_object.borrow_mut();
        integer_object.initialize(Vec::new(), integer_vtable);
        let number_object = integer_object.get_super_object().unwrap().clone();
        let mut number_object = number_object.borrow_mut();
        number_object.initialize(Vec::new(), number_vtable);
    }
}

create_type_ops!(u8, u8_add, u8_sub, u8_mul, u8_div, u8_mod);
create_integer_ops!(u8, u8_divides, u8_shr, u8_shl, u8_and, u8_or, u8_xor);


