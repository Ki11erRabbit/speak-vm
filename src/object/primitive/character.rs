use crate::object::Method;
use crate::object::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use crate::object::primitive::PrimitiveObject;
use crate::object::ObjectBox;
use crate::object::VTable;
use crate::object::ContextData;
use crate::object::Fault;






pub struct CharacterObject {}

impl CharacterObject {
    pub fn make_object(parent: ObjectBox, value: char) -> ObjectBox {
        Rc::new(RefCell::new(PrimitiveObject::new(Some(parent), value)))
    }
    pub fn make_object_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert("equals".to_string(), Arc::new(Method::RustMethod { fun: Box::new(character_equals) }));
        methods.insert("to_string".to_string(), Arc::new(Method::RustMethod { fun: Box::new(character_to_string) }));
        methods.insert("order".to_string(), Arc::new(Method::RustMethod { fun: Box::new(character_order) }));
        VTable::new(methods)
    }
    pub fn make_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert("to_lowercase".to_string(), Arc::new(Method::RustMethod { fun: Box::new(char_to_lowercase) }));
        methods.insert("to_uppercase".to_string(), Arc::new(Method::RustMethod { fun: Box::new(char_to_uppercase) }));
        methods.insert("is_lowercase".to_string(), Arc::new(Method::RustMethod { fun: Box::new(char_is_lowercase) }));
        methods.insert("is_uppercase".to_string(), Arc::new(Method::RustMethod { fun: Box::new(char_is_uppercase) }));
        methods.insert("is_alphabetic".to_string(), Arc::new(Method::RustMethod { fun: Box::new(char_is_alphabetic) }));
        methods.insert("is_alphanumeric".to_string(), Arc::new(Method::RustMethod { fun: Box::new(char_is_alphanumeric) }));
        methods.insert("is_numeric".to_string(), Arc::new(Method::RustMethod { fun: Box::new(char_is_numeric) }));
        methods.insert("is_whitespace".to_string(), Arc::new(Method::RustMethod { fun: Box::new(char_is_whitespace) }));
        VTable::new(methods)
    }
}

impl Object for PrimitiveObject<char> {
    fn get_vtable(&self) -> &VTable {
        &self.vtable
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        self.super_object.clone()
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox> {
        panic!("Char objects do not have fields")
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox) {
        panic!("Char objects do not have fields")
    }
    fn size(&self) -> Option<usize> {
        None
    }
    fn duplicate(&self) -> ObjectBox {
        let character = CharacterObject::make_object(self.super_object.clone().unwrap().borrow().duplicate(), self.data);
        let mut chr = character.borrow_mut();
        chr.initialize(Vec::new(), self.vtable.clone());
        drop(chr);
        character
    }
    fn initialize(&mut self, _: Vec<ObjectBox>, vtable: VTable) {
        let object_vtable = CharacterObject::make_object_vtable();
        let parent = self.super_object.clone().unwrap();
        let mut parent = parent.borrow_mut();
        parent.initialize(Vec::new(), object_vtable);
        self.vtable.extend(CharacterObject::make_vtable());
        self.vtable.extend(vtable);
    }
}
      


fn character_equals(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    let other = context.get_argument(0).unwrap();
    let other = other.borrow();
    match (object.downcast_ref::<PrimitiveObject<char>>(), other.downcast_ref::<PrimitiveObject<char>>()) {
        (Some(obj), Some(other)) => Ok(Some(crate::object::create_boolean(obj.data == other.data))),
        _ => Err(Fault::InvalidType)
    }
}

fn character_to_string(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    match object.downcast_ref::<PrimitiveObject<char>>() {
        Some(obj) => Ok(Some(crate::object::create_string(obj.data.to_string()))),
        _ => Err(Fault::InvalidType)
    }
}

fn character_order(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    let other = context.get_argument(0).unwrap();
    let other = other.borrow();
    match (object.downcast_ref::<PrimitiveObject<char>>(), other.downcast_ref::<PrimitiveObject<char>>()) {
        (Some(obj), Some(other)) => if obj.data > other.data {
                Ok(Some(crate::object::create_i8(1)))
            } else if obj.data < other.data {
                Ok(Some(crate::object::create_i8(-1)))
            } else {
                Ok(Some(crate::object::create_i8(0)))
            },
        _ => Err(Fault::InvalidType)
    }
}

fn char_to_lowercase(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    match object.downcast_ref::<PrimitiveObject<char>>() {
        Some(obj) => Ok(Some(crate::object::create_character(obj.data.to_lowercase().next().unwrap_or(obj.data)))),
        _ => Err(Fault::InvalidType)
    }
}

fn char_to_uppercase(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    match object.downcast_ref::<PrimitiveObject<char>>() {
        Some(obj) => Ok(Some(crate::object::create_character(obj.data.to_uppercase().next().unwrap_or(obj.data)))),
        _ => Err(Fault::InvalidType)
    }
}

fn char_is_lowercase(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    match object.downcast_ref::<PrimitiveObject<char>>() {
        Some(obj) => Ok(Some(crate::object::create_boolean(obj.data.is_lowercase()))),
        _ => Err(Fault::InvalidType)
    }
}

fn char_is_uppercase(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    match object.downcast_ref::<PrimitiveObject<char>>() {
        Some(obj) => Ok(Some(crate::object::create_boolean(obj.data.is_uppercase()))),
        _ => Err(Fault::InvalidType)
    }
}

fn char_is_alphabetic(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    match object.downcast_ref::<PrimitiveObject<char>>() {
        Some(obj) => Ok(Some(crate::object::create_boolean(obj.data.is_alphabetic()))),
        _ => Err(Fault::InvalidType)
    }
}

fn char_is_alphanumeric(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    match object.downcast_ref::<PrimitiveObject<char>>() {
        Some(obj) => Ok(Some(crate::object::create_boolean(obj.data.is_alphanumeric()))),
        _ => Err(Fault::InvalidType)
    }
}

fn char_is_numeric(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    match object.downcast_ref::<PrimitiveObject<char>>() {
        Some(obj) => Ok(Some(crate::object::create_boolean(obj.data.is_numeric()))),
        _ => Err(Fault::InvalidType)
    }
}

fn char_is_whitespace(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    match object.downcast_ref::<PrimitiveObject<char>>() {
        Some(obj) => Ok(Some(crate::object::create_boolean(obj.data.is_whitespace()))),
        _ => Err(Fault::InvalidType)
    }
}

