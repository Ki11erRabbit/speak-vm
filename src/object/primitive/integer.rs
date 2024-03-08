use super::{Method, ObjectBox};
use std::collections::HashMap;
use crate::object::Object;
use crate::object::ContextData;
use std::sync::Arc;
use super::Fault;
use num_traits::Zero;
use crate::{create_type_ops, primitive_base_ops};
use crate::object::primitive::PrimitiveObject;
use num_integer::Integer;
use crate::object::create_boolean;
use crate::object::VTable;


trait Abs {
    fn abs(self) -> Self;
}

impl Abs for u64 {
    fn abs(self) -> u64 {
        self
    }
}
impl Abs for u32 {
    fn abs(self) -> u32 {
        self
    }
}
impl Abs for u16 {
    fn abs(self) -> u16 {
        self
    }
}
impl Abs for u8 {
    fn abs(self) -> u8 {
        self
    }
}


pub struct IntegerObject {
    super_object: Option<ObjectBox>,
    vtable: VTable,
}

impl IntegerObject {
    pub fn make_object(parent: ObjectBox) -> ObjectBox {
        let out = ObjectBox::new(IntegerObject{super_object: Some(parent), vtable: VTable::new_empty()});
        return out;
    }
    pub fn make_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("divides"), Arc::new(Method::RustMethod { fun: Box::new(integer_divides) }));
        methods.insert(String::from("shift_right"), Arc::new(Method::RustMethod { fun: Box::new(integer_shift_right) }));
        methods.insert(String::from("shift_left"), Arc::new(Method::RustMethod { fun: Box::new(integer_shift_left) }));
        methods.insert(String::from("and"), Arc::new(Method::RustMethod { fun: Box::new(integer_bitwise_and) }));
        methods.insert(String::from("or"), Arc::new(Method::RustMethod { fun: Box::new(integer_bitwise_or) }));
        methods.insert(String::from("xor"), Arc::new(Method::RustMethod { fun: Box::new(integer_bitwise_xor) }));
        VTable::new(methods)
    }
}


impl Object for IntegerObject {
    fn get_vtable(&self) -> &VTable {
        &self.vtable
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
        let integer = IntegerObject::make_object(self.super_object.clone().unwrap().borrow().duplicate());
        let mut int = integer.borrow_mut();
        int.initialize(Vec::new(), self.vtable.clone());
        drop(int);
        integer
    }
    fn initialize(&mut self, _: Vec<ObjectBox>, vtable: crate::object::VTable) {
        let integer_vtable = IntegerObject::make_vtable();
        self.vtable.extend(integer_vtable);
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

    pub fn make_object(parent: ObjectBox, data: i64) -> ObjectBox {
        let out = ObjectBox::new(PrimitiveObject::new(Some(parent), data));
        return out;
    }
    pub fn make_object_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("equals"), Arc::new(Method::RustMethod { fun: Box::new(i64_equals) }));
        methods.insert(String::from("to_string"), Arc::new(Method::RustMethod { fun: Box::new(i64_to_string) }));
        methods.insert(String::from("order"), Arc::new(Method::RustMethod { fun: Box::new(i64_order) }));
        return VTable::new(methods);
    }
    pub fn make_number_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(i64_add) }));
        methods.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(i64_sub) }));
        methods.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(i64_mul) }));
        methods.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(i64_div) }));
        methods.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(i64_mod) }));
        methods.insert(String::from("abs"), Arc::new(Method::RustMethod { fun: Box::new(i64_abs) }));
        methods.insert(String::from("pow"), Arc::new(Method::RustMethod { fun: Box::new(i64_pow) }));
        methods.insert(String::from("is_zero"), Arc::new(Method::RustMethod { fun: Box::new(i64_is_zero) }));
        VTable::new(methods)
    }
    pub fn make_integer_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("divides"), Arc::new(Method::RustMethod { fun: Box::new(i64_divides) }));
        methods.insert(String::from("shift_right"), Arc::new(Method::RustMethod { fun: Box::new(i64_shr) }));
        methods.insert(String::from("shift_left"), Arc::new(Method::RustMethod { fun: Box::new(i64_shl) }));
        methods.insert(String::from("and"), Arc::new(Method::RustMethod { fun: Box::new(i64_and) }));
        methods.insert(String::from("or"), Arc::new(Method::RustMethod { fun: Box::new(i64_or) }));
        methods.insert(String::from("xor"), Arc::new(Method::RustMethod { fun: Box::new(i64_xor) }));
        VTable::new(methods)
    }
}


impl Object for PrimitiveObject<i64> {
    fn get_vtable(&self) -> &VTable {
        &self.vtable
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
        let integer = I64Object::make_object(self.super_object.clone().unwrap().borrow().duplicate(), self.data);
        let mut int = integer.borrow_mut();
        int.initialize(Vec::new(), self.vtable.clone());
        drop(int);
        integer
    }
    fn initialize(&mut self, _: Vec<ObjectBox>, vtable: VTable) {
        let object_vtable = I64Object::make_object_vtable();
        let number_vtable = I64Object::make_number_vtable();
        let integer_vtable = I64Object::make_integer_vtable();

        let integer_object = self.get_super_object().unwrap().clone();
        let mut integer_object = integer_object.borrow_mut();
        integer_object.initialize(Vec::new(), integer_vtable);
        let number_object = integer_object.get_super_object().unwrap().clone();
        let mut number_object = number_object.borrow_mut();
        number_object.initialize(Vec::new(), number_vtable);
        let object_object = number_object.get_super_object().unwrap().clone();
        let mut object_object = object_object.borrow_mut();
        object_object.initialize(Vec::new(), object_vtable);
        self.vtable.extend(vtable);
    }
}

create_type_ops!(i64, i64_add, i64_sub, i64_mul, i64_div, i64_mod, i64_abs, i64_pow, i64_is_zero);
create_integer_ops!(i64, i64_divides, i64_shr, i64_shl, i64_and, i64_or, i64_xor);
primitive_base_ops!(i64, i64_equals, i64_to_string, i64_order);

pub struct U64Object {
}

impl U64Object {
    pub fn make_object(parent: ObjectBox, data: u64) -> ObjectBox {
        let out = ObjectBox::new(PrimitiveObject::new(Some(parent), data));
        return out;
    }
    pub fn make_object_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("equals"), Arc::new(Method::RustMethod { fun: Box::new(u64_equals) }));
        methods.insert(String::from("to_string"), Arc::new(Method::RustMethod { fun: Box::new(u64_to_string) }));
        methods.insert(String::from("order"), Arc::new(Method::RustMethod { fun: Box::new(u64_order) }));
        return VTable::new(methods);
    }
    pub fn make_number_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(u64_add) }));
        methods.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(u64_sub) }));
        methods.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(u64_mul) }));
        methods.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(u64_div) }));
        methods.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(u64_mod) }));
        methods.insert(String::from("abs"), Arc::new(Method::RustMethod { fun: Box::new(u64_abs) }));
        methods.insert(String::from("pow"), Arc::new(Method::RustMethod { fun: Box::new(u64_pow) }));
        methods.insert(String::from("is_zero"), Arc::new(Method::RustMethod { fun: Box::new(u64_is_zero) }));
        VTable::new(methods)
    }
    pub fn make_integer_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("divides"), Arc::new(Method::RustMethod { fun: Box::new(u64_divides) }));
        methods.insert(String::from("shift_right"), Arc::new(Method::RustMethod { fun: Box::new(u64_shr) }));
        methods.insert(String::from("shift_left"), Arc::new(Method::RustMethod { fun: Box::new(u64_shl) }));
        methods.insert(String::from("and"), Arc::new(Method::RustMethod { fun: Box::new(u64_and) }));
        methods.insert(String::from("or"), Arc::new(Method::RustMethod { fun: Box::new(u64_or) }));
        methods.insert(String::from("xor"), Arc::new(Method::RustMethod { fun: Box::new(u64_xor) }));
        VTable::new(methods)
    }
}


impl Object for PrimitiveObject<u64> {
    fn get_vtable(&self) -> &VTable {
        &self.vtable
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
        let integer = U64Object::make_object(self.super_object.clone().unwrap().borrow().duplicate(), self.data);
        let mut int = integer.borrow_mut();
        int.initialize(Vec::new(), self.vtable.clone());
        drop(int);
        integer
    }
    fn initialize(&mut self, _: Vec<ObjectBox>, vtable: VTable) {
        let object_vtable = U64Object::make_object_vtable();
        let number_vtable = U64Object::make_number_vtable();
        let integer_vtable = U64Object::make_integer_vtable();

        let integer_object = self.get_super_object().unwrap().clone();
        let mut integer_object = integer_object.borrow_mut();
        integer_object.initialize(Vec::new(), integer_vtable);
        let number_object = integer_object.get_super_object().unwrap().clone();
        let mut number_object = number_object.borrow_mut();
        number_object.initialize(Vec::new(), number_vtable);
        let object_object = number_object.get_super_object().unwrap().clone();
        let mut object_object = object_object.borrow_mut();
        object_object.initialize(Vec::new(), object_vtable);
        self.vtable.extend(vtable);
    }
}

create_type_ops!(u64, u64_add, u64_sub, u64_mul, u64_div, u64_mod, u64_abs, u64_pow, u64_is_zero);
create_integer_ops!(u64, u64_divides, u64_shr, u64_shl, u64_and, u64_or, u64_xor);
primitive_base_ops!(u64, u64_equals, u64_to_string, u64_order);

pub struct I32Object {
}

impl I32Object {
    pub fn make_object(parent: ObjectBox, data: i32) -> ObjectBox {
        let out = ObjectBox::new(PrimitiveObject::new(Some(parent), data));
        return out;
    }
    pub fn make_object_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("equals"), Arc::new(Method::RustMethod { fun: Box::new(i32_equals) }));
        methods.insert(String::from("to_string"), Arc::new(Method::RustMethod { fun: Box::new(i32_to_string) }));
        methods.insert(String::from("order"), Arc::new(Method::RustMethod { fun: Box::new(i32_order) }));
        return VTable::new(methods);
    }
    pub fn make_number_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(i32_add) }));
        methods.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(i32_sub) }));
        methods.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(i32_mul) }));
        methods.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(i32_div) }));
        methods.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(i32_mod) }));
        methods.insert(String::from("abs"), Arc::new(Method::RustMethod { fun: Box::new(i32_abs) }));
        methods.insert(String::from("pow"), Arc::new(Method::RustMethod { fun: Box::new(i32_pow) }));
        methods.insert(String::from("is_zero"), Arc::new(Method::RustMethod { fun: Box::new(i32_is_zero) }));
        VTable::new(methods)
    }
    pub fn make_integer_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("divides"), Arc::new(Method::RustMethod { fun: Box::new(i32_divides) }));
        methods.insert(String::from("shift_right"), Arc::new(Method::RustMethod { fun: Box::new(i32_shr) }));
        methods.insert(String::from("shift_left"), Arc::new(Method::RustMethod { fun: Box::new(i32_shl) }));
        methods.insert(String::from("and"), Arc::new(Method::RustMethod { fun: Box::new(i32_and) }));
        methods.insert(String::from("or"), Arc::new(Method::RustMethod { fun: Box::new(i32_or) }));
        methods.insert(String::from("xor"), Arc::new(Method::RustMethod { fun: Box::new(i32_xor) }));
        VTable::new(methods)
    }
}


impl Object for PrimitiveObject<i32> {
    fn get_vtable(&self) -> &VTable {
        &self.vtable
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
        let integer = I32Object::make_object(self.super_object.clone().unwrap().borrow().duplicate(), self.data);
        let mut int = integer.borrow_mut();
        int.initialize(Vec::new(), self.vtable.clone());
        drop(int);
        integer
    }
    fn initialize(&mut self, _: Vec<ObjectBox>, vtable: VTable) {
        let object_vtable = I32Object::make_object_vtable();
        let number_vtable = I32Object::make_number_vtable();
        let integer_vtable = I32Object::make_integer_vtable();

        let integer_object = self.get_super_object().unwrap().clone();
        let mut integer_object = integer_object.borrow_mut();
        integer_object.initialize(Vec::new(), integer_vtable);
        let number_object = integer_object.get_super_object().unwrap().clone();
        let mut number_object = number_object.borrow_mut();
        number_object.initialize(Vec::new(), number_vtable);
        let object_object = number_object.get_super_object().unwrap().clone();
        let mut object_object = object_object.borrow_mut();
        object_object.initialize(Vec::new(), object_vtable);
        self.vtable.extend(vtable);
    }
}

create_type_ops!(i32, i32_add, i32_sub, i32_mul, i32_div, i32_mod, i32_abs, i32_pow, i32_is_zero);
create_integer_ops!(i32, i32_divides, i32_shr, i32_shl, i32_and, i32_or, i32_xor);
primitive_base_ops!(i32, i32_equals, i32_to_string, i32_order);

pub struct U32Object {
}

impl U32Object {
    pub fn make_object(parent: ObjectBox, data: u32) -> ObjectBox {
        let out = ObjectBox::new(PrimitiveObject::new(Some(parent), data));
        return out;
    }
    pub fn make_object_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("equals"), Arc::new(Method::RustMethod { fun: Box::new(u32_equals) }));
        methods.insert(String::from("to_string"), Arc::new(Method::RustMethod { fun: Box::new(u32_to_string) }));
        methods.insert(String::from("order"), Arc::new(Method::RustMethod { fun: Box::new(u32_order) }));
        return VTable::new(methods);
    }
    pub fn make_number_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(u32_add) }));
        methods.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(u32_sub) }));
        methods.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(u32_mul) }));
        methods.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(u32_div) }));
        methods.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(u32_mod) }));
        methods.insert(String::from("abs"), Arc::new(Method::RustMethod { fun: Box::new(u32_abs) }));
        methods.insert(String::from("pow"), Arc::new(Method::RustMethod { fun: Box::new(u32_pow) }));
        methods.insert(String::from("is_zero"), Arc::new(Method::RustMethod { fun: Box::new(u32_is_zero) }));
        VTable::new(methods)
    }
    pub fn make_integer_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("divides"), Arc::new(Method::RustMethod { fun: Box::new(u32_divides) }));
        methods.insert(String::from("shift_right"), Arc::new(Method::RustMethod { fun: Box::new(u32_shr) }));
        methods.insert(String::from("shift_left"), Arc::new(Method::RustMethod { fun: Box::new(u32_shl) }));
        methods.insert(String::from("and"), Arc::new(Method::RustMethod { fun: Box::new(u32_and) }));
        methods.insert(String::from("or"), Arc::new(Method::RustMethod { fun: Box::new(u32_or) }));
        methods.insert(String::from("xor"), Arc::new(Method::RustMethod { fun: Box::new(u32_xor) }));
        VTable::new(methods)
    }
}


impl Object for PrimitiveObject<u32> {
    fn get_vtable(&self) -> &VTable {
        &self.vtable
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
        let integer = U32Object::make_object(self.super_object.clone().unwrap().borrow().duplicate(), self.data);
        let mut int = integer.borrow_mut();
        int.initialize(Vec::new(), self.vtable.clone());
        drop(int);
        integer
    }
    fn initialize(&mut self, _: Vec<ObjectBox>, vtable: VTable) {
        let object_vtable = U32Object::make_object_vtable();
        let number_vtable = U32Object::make_number_vtable();
        let integer_vtable = U32Object::make_integer_vtable();

        let integer_object = self.get_super_object().unwrap().clone();
        let mut integer_object = integer_object.borrow_mut();
        integer_object.initialize(Vec::new(), integer_vtable);
        let number_object = integer_object.get_super_object().unwrap().clone();
        let mut number_object = number_object.borrow_mut();
        number_object.initialize(Vec::new(), number_vtable);
        let object_object = number_object.get_super_object().unwrap().clone();
        let mut object_object = object_object.borrow_mut();
        object_object.initialize(Vec::new(), object_vtable);
        self.vtable.extend(vtable);
    }
}

create_type_ops!(u32, u32_add, u32_sub, u32_mul, u32_div, u32_mod, u32_abs, u32_pow, u32_is_zero);
create_integer_ops!(u32, u32_divides, u32_shr, u32_shl, u32_and, u32_or, u32_xor);
primitive_base_ops!(u32, u32_equals, u32_to_string, u32_order);

pub struct I16Object {
}

impl I16Object {
    pub fn make_object(parent: ObjectBox, data: i16) -> ObjectBox {
        let out = ObjectBox::new(PrimitiveObject::new(Some(parent), data));
        return out;
    }
    pub fn make_object_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("equals"), Arc::new(Method::RustMethod { fun: Box::new(i16_equals) }));
        methods.insert(String::from("to_string"), Arc::new(Method::RustMethod { fun: Box::new(i16_to_string) }));
        methods.insert(String::from("order"), Arc::new(Method::RustMethod { fun: Box::new(i16_order) }));
        return VTable::new(methods);
    }
    pub fn make_number_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(i16_add) }));
        methods.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(i16_sub) }));
        methods.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(i16_mul) }));
        methods.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(i16_div) }));
        methods.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(i16_mod) }));
        methods.insert(String::from("abs"), Arc::new(Method::RustMethod { fun: Box::new(i16_abs) }));
        methods.insert(String::from("pow"), Arc::new(Method::RustMethod { fun: Box::new(i16_pow) }));
        methods.insert(String::from("is_zero"), Arc::new(Method::RustMethod { fun: Box::new(i16_is_zero) }));
        VTable::new(methods)
    }
    pub fn make_integer_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("divides"), Arc::new(Method::RustMethod { fun: Box::new(i16_divides) }));
        methods.insert(String::from("shift_right"), Arc::new(Method::RustMethod { fun: Box::new(i16_shr) }));
        methods.insert(String::from("shift_left"), Arc::new(Method::RustMethod { fun: Box::new(i16_shl) }));
        methods.insert(String::from("and"), Arc::new(Method::RustMethod { fun: Box::new(i16_and) }));
        methods.insert(String::from("or"), Arc::new(Method::RustMethod { fun: Box::new(i16_or) }));
        methods.insert(String::from("xor"), Arc::new(Method::RustMethod { fun: Box::new(i16_xor) }));
        VTable::new(methods)
    }
}


impl Object for PrimitiveObject<i16> {
    fn get_vtable(&self) -> &VTable {
        &self.vtable
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
        let integer = I16Object::make_object(self.super_object.clone().unwrap().borrow().duplicate(), self.data);
        let mut int = integer.borrow_mut();
        int.initialize(Vec::new(), self.vtable.clone());
        drop(int);
        integer
    }
    fn initialize(&mut self, _: Vec<ObjectBox>, vtable: VTable) {
        let object_vtable = I16Object::make_object_vtable();
        let number_vtable = I16Object::make_number_vtable();
        let integer_vtable = I16Object::make_integer_vtable();

        let integer_object = self.get_super_object().unwrap().clone();
        let mut integer_object = integer_object.borrow_mut();
        integer_object.initialize(Vec::new(), integer_vtable);
        let number_object = integer_object.get_super_object().unwrap().clone();
        let mut number_object = number_object.borrow_mut();
        number_object.initialize(Vec::new(), number_vtable);
        let object_object = number_object.get_super_object().unwrap().clone();
        let mut object_object = object_object.borrow_mut();
        object_object.initialize(Vec::new(), object_vtable);
        self.vtable.extend(vtable);
    }
}

create_type_ops!(i16, i16_add, i16_sub, i16_mul, i16_div, i16_mod, i16_abs, i16_pow, i16_is_zero);
create_integer_ops!(i16, i16_divides, i16_shr, i16_shl, i16_and, i16_or, i16_xor);
primitive_base_ops!(i16, i16_equals, i16_to_string, i16_order);

pub struct U16Object {
}

impl U16Object {
    pub fn make_object(parent: ObjectBox, data: u16) -> ObjectBox {
        let out = ObjectBox::new(PrimitiveObject::new(Some(parent), data));
        return out;
    }
    pub fn make_object_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("equals"), Arc::new(Method::RustMethod { fun: Box::new(u16_equals) }));
        methods.insert(String::from("to_string"), Arc::new(Method::RustMethod { fun: Box::new(u16_to_string) }));
        methods.insert(String::from("order"), Arc::new(Method::RustMethod { fun: Box::new(u16_order) }));
        return VTable::new(methods);
    }
    pub fn make_number_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(u16_add) }));
        methods.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(u16_sub) }));
        methods.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(u16_mul) }));
        methods.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(u16_div) }));
        methods.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(u16_mod) }));
        methods.insert(String::from("abs"), Arc::new(Method::RustMethod { fun: Box::new(u16_abs) }));
        methods.insert(String::from("pow"), Arc::new(Method::RustMethod { fun: Box::new(u16_pow) }));
        methods.insert(String::from("is_zero"), Arc::new(Method::RustMethod { fun: Box::new(u16_is_zero) }));
        VTable::new(methods)
    }
    pub fn make_integer_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("divides"), Arc::new(Method::RustMethod { fun: Box::new(u16_divides) }));
        methods.insert(String::from("shift_right"), Arc::new(Method::RustMethod { fun: Box::new(u16_shr) }));
        methods.insert(String::from("shift_left"), Arc::new(Method::RustMethod { fun: Box::new(u16_shl) }));
        methods.insert(String::from("and"), Arc::new(Method::RustMethod { fun: Box::new(u16_and) }));
        methods.insert(String::from("or"), Arc::new(Method::RustMethod { fun: Box::new(u16_or) }));
        methods.insert(String::from("xor"), Arc::new(Method::RustMethod { fun: Box::new(u16_xor) }));
        VTable::new(methods)
    }
}


impl Object for PrimitiveObject<u16> {
    fn get_vtable(&self) -> &VTable {
        &self.vtable
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
        let integer = U16Object::make_object(self.super_object.clone().unwrap().borrow().duplicate(), self.data);
        let mut int = integer.borrow_mut();
        int.initialize(Vec::new(), self.vtable.clone());
        drop(int);
        integer
    }
    fn initialize(&mut self, _: Vec<ObjectBox>, vtable: VTable) {
        let object_vtable = U16Object::make_object_vtable();
        let number_vtable = U16Object::make_number_vtable();
        let integer_vtable = U16Object::make_integer_vtable();

        let integer_object = self.get_super_object().unwrap().clone();
        let mut integer_object = integer_object.borrow_mut();
        integer_object.initialize(Vec::new(), integer_vtable);
        let number_object = integer_object.get_super_object().unwrap().clone();
        let mut number_object = number_object.borrow_mut();
        number_object.initialize(Vec::new(), number_vtable);
        let object_object = number_object.get_super_object().unwrap().clone();
        let mut object_object = object_object.borrow_mut();
        object_object.initialize(Vec::new(), object_vtable);
        self.vtable.extend(vtable);
    }
}

create_type_ops!(u16, u16_add, u16_sub, u16_mul, u16_div, u16_mod, u16_abs, u16_pow, u16_is_zero);
create_integer_ops!(u16, u16_divides, u16_shr, u16_shl, u16_and, u16_or, u16_xor);
primitive_base_ops!(u16, u16_equals, u16_to_string, u16_order);

pub struct I8Object {
}

impl I8Object {
    pub fn make_object(parent: ObjectBox, data: i8) -> ObjectBox {
        let out = ObjectBox::new(PrimitiveObject::new(Some(parent), data));
        return out;
    }
    pub fn make_object_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("equals"), Arc::new(Method::RustMethod { fun: Box::new(i8_equals) }));
        methods.insert(String::from("to_string"), Arc::new(Method::RustMethod { fun: Box::new(i8_to_string) }));
        methods.insert(String::from("order"), Arc::new(Method::RustMethod { fun: Box::new(i8_order) }));
        return VTable::new(methods);
    }
    pub fn make_number_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(i8_add) }));
        methods.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(i8_sub) }));
        methods.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(i8_mul) }));
        methods.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(i8_div) }));
        methods.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(i8_mod) }));
        methods.insert(String::from("abs"), Arc::new(Method::RustMethod { fun: Box::new(i8_abs) }));
        methods.insert(String::from("pow"), Arc::new(Method::RustMethod { fun: Box::new(i8_pow) }));
        methods.insert(String::from("is_zero"), Arc::new(Method::RustMethod { fun: Box::new(i8_is_zero) }));
        VTable::new(methods)
    }
    pub fn make_integer_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("divides"), Arc::new(Method::RustMethod { fun: Box::new(i8_divides) }));
        methods.insert(String::from("shift_right"), Arc::new(Method::RustMethod { fun: Box::new(i8_shr) }));
        methods.insert(String::from("shift_left"), Arc::new(Method::RustMethod { fun: Box::new(i8_shl) }));
        methods.insert(String::from("and"), Arc::new(Method::RustMethod { fun: Box::new(i8_and) }));
        methods.insert(String::from("or"), Arc::new(Method::RustMethod { fun: Box::new(i8_or) }));
        methods.insert(String::from("xor"), Arc::new(Method::RustMethod { fun: Box::new(i8_xor) }));
        VTable::new(methods)
    }
}


impl Object for PrimitiveObject<i8> {
    fn get_vtable(&self) -> &VTable {
        &self.vtable
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
        let integer = I8Object::make_object(self.super_object.clone().unwrap().borrow().duplicate(), self.data);
        let mut int = integer.borrow_mut();
        int.initialize(Vec::new(), self.vtable.clone());
        drop(int);
        integer
    }
    fn initialize(&mut self, _: Vec<ObjectBox>, vtable: VTable) {
        let object_vtable = I8Object::make_object_vtable();
        let number_vtable = I8Object::make_number_vtable();
        let integer_vtable = I8Object::make_integer_vtable();

        let integer_object = self.get_super_object().unwrap().clone();
        let mut integer_object = integer_object.borrow_mut();
        integer_object.initialize(Vec::new(), integer_vtable);
        let number_object = integer_object.get_super_object().unwrap().clone();
        let mut number_object = number_object.borrow_mut();
        number_object.initialize(Vec::new(), number_vtable);
        let object_object = number_object.get_super_object().unwrap().clone();
        let mut object_object = object_object.borrow_mut();
        object_object.initialize(Vec::new(), object_vtable);
        self.vtable.extend(vtable);
    }
}

create_type_ops!(i8, i8_add, i8_sub, i8_mul, i8_div, i8_mod, i8_abs, i8_pow, i8_is_zero);
create_integer_ops!(i8, i8_divides, i8_shr, i8_shl, i8_and, i8_or, i8_xor);
primitive_base_ops!(i8, i8_equals, i8_to_string, i8_order);

pub struct U8Object {
}

impl U8Object {
    pub fn make_object(parent: ObjectBox, data: u8) -> ObjectBox {
        let out = ObjectBox::new(PrimitiveObject::new(Some(parent), data));
        return out;
    }
    pub fn make_object_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("equals"), Arc::new(Method::RustMethod { fun: Box::new(u8_equals) }));
        methods.insert(String::from("to_string"), Arc::new(Method::RustMethod { fun: Box::new(u8_to_string) }));
        methods.insert(String::from("order"), Arc::new(Method::RustMethod { fun: Box::new(u8_order) }));
        return VTable::new(methods);
    }
    pub fn make_number_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(u8_add) }));
        methods.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(u8_sub) }));
        methods.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(u8_mul) }));
        methods.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(u8_div) }));
        methods.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(u8_mod) }));
        methods.insert(String::from("abs"), Arc::new(Method::RustMethod { fun: Box::new(u8_abs) }));
        methods.insert(String::from("pow"), Arc::new(Method::RustMethod { fun: Box::new(u8_pow) }));
        methods.insert(String::from("is_zero"), Arc::new(Method::RustMethod { fun: Box::new(u8_is_zero) }));
        VTable::new(methods)
    }
    pub fn make_integer_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("divides"), Arc::new(Method::RustMethod { fun: Box::new(u8_divides) }));
        methods.insert(String::from("shift_right"), Arc::new(Method::RustMethod { fun: Box::new(u8_shr) }));
        methods.insert(String::from("shift_left"), Arc::new(Method::RustMethod { fun: Box::new(u8_shl) }));
        methods.insert(String::from("and"), Arc::new(Method::RustMethod { fun: Box::new(u8_and) }));
        methods.insert(String::from("or"), Arc::new(Method::RustMethod { fun: Box::new(u8_or) }));
        methods.insert(String::from("xor"), Arc::new(Method::RustMethod { fun: Box::new(u8_xor) }));
        VTable::new(methods)
    }
}


impl Object for PrimitiveObject<u8> {
    fn get_vtable(&self) -> &VTable {
        &self.vtable
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
        let integer = U8Object::make_object(self.super_object.clone().unwrap().borrow().duplicate(), self.data);
        let mut int = integer.borrow_mut();
        int.initialize(Vec::new(), self.vtable.clone());
        drop(int);
        integer
    }
    fn initialize(&mut self, _: Vec<ObjectBox>, vtable: VTable) {
        let object_vtable = U8Object::make_object_vtable();
        let number_vtable = U8Object::make_number_vtable();
        let integer_vtable = U8Object::make_integer_vtable();

        let integer_object = self.get_super_object().unwrap().clone();
        let mut integer_object = integer_object.borrow_mut();
        integer_object.initialize(Vec::new(), integer_vtable);
        let number_object = integer_object.get_super_object().unwrap().clone();
        let mut number_object = number_object.borrow_mut();
        number_object.initialize(Vec::new(), number_vtable);
        let object_object = number_object.get_super_object().unwrap().clone();
        let mut object_object = object_object.borrow_mut();
        object_object.initialize(Vec::new(), object_vtable);
        self.vtable.extend(vtable);
    }
}

create_type_ops!(u8, u8_add, u8_sub, u8_mul, u8_div, u8_mod, u8_abs, u8_pow, u8_is_zero);
create_integer_ops!(u8, u8_divides, u8_shr, u8_shl, u8_and, u8_or, u8_xor);
primitive_base_ops!(u8, u8_equals, u8_to_string, u8_order);

