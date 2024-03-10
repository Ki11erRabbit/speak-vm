use std::collections::HashMap;
use std::sync::Arc;
use super::vector::VectorObject;
use super::{Object, ObjectBox, VTable, Method, ContextData, Fault};
use crate::object::primitive::PrimitiveObject;




pub struct StringObject {
    super_object: Option<ObjectBox>,
    vtable: VTable,
    pub value: String,
}



impl StringObject {
    pub fn make_object(parent: ObjectBox, value: String) -> ObjectBox {
        let string = StringObject {
            super_object: Some(parent),
            value,
            vtable: VTable::new_empty(),
        };
        ObjectBox::new(string)
    }
    fn make_object_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("equals"), Arc::new(Method::RustMethod { fun: Box::new(string_equals) }));
        methods.insert(String::from("to_string"), Arc::new(Method::RustMethod { fun: Box::new(string_to_string) }));
        methods.insert(String::from("order"), Arc::new(Method::RustMethod { fun: Box::new(string_order) }));
        VTable::new(methods)
    }
    pub fn make_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("length"), Arc::new(Method::RustMethod { fun: Box::new(string_length) }));
        methods.insert(String::from("to_lowercase"), Arc::new(Method::RustMethod { fun: Box::new(string_to_lowercase) }));
        methods.insert(String::from("to_uppercase"), Arc::new(Method::RustMethod { fun: Box::new(string_to_uppercase) }));
        methods.insert(String::from("trim"), Arc::new(Method::RustMethod { fun: Box::new(string_trim) }));
        methods.insert(String::from("trim_start"), Arc::new(Method::RustMethod { fun: Box::new(string_trim_start) }));
        methods.insert(String::from("trim_end"), Arc::new(Method::RustMethod { fun: Box::new(string_trim_end) }));
        methods.insert(String::from("contains"), Arc::new(Method::RustMethod { fun: Box::new(string_contains) }));
        methods.insert(String::from("to_vector"), Arc::new(Method::RustMethod { fun: Box::new(string_to_vector) }));
        methods.insert(String::from("split"), Arc::new(Method::RustMethod { fun: Box::new(string_split) }));
        methods.insert(String::from("get"), Arc::new(Method::RustMethod { fun: Box::new(string_get) }));
        methods.insert(String::from("set"), Arc::new(Method::RustMethod { fun: Box::new(string_set) }));
        methods.insert(String::from("push"), Arc::new(Method::RustMethod { fun: Box::new(string_push) }));
        methods.insert(String::from("pop"), Arc::new(Method::RustMethod { fun: Box::new(string_pop) }));
        methods.insert(String::from("push_char"), Arc::new(Method::RustMethod { fun: Box::new(string_push_char) }));
        methods.insert(String::from("concat"), Arc::new(Method::RustMethod { fun: Box::new(string_concat) }));
        VTable::new(methods)
    }
}


impl Object for StringObject {
    fn get_vtable(&self) -> &VTable {
        &self.vtable
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        self.super_object.clone()
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox> {
        panic!("String does not have fields");
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox) {
        panic!("String does not have fields");
    }
    fn size(&self) -> Option<usize> {
        Some(self.value.len())
    }
    fn duplicate(&self) -> ObjectBox {
        let string = StringObject::make_object(self.super_object.clone().unwrap().borrow().duplicate(), self.value.clone());
        let mut str_obj = string.borrow_mut();
        str_obj.initialize(Vec::new(), self.vtable.clone());
        drop(str_obj);
        string
    }
    fn initialize(&mut self, args: Vec<ObjectBox>, vtable: VTable) {
        self.vtable.extend(StringObject::make_vtable());
        self.vtable.extend(vtable);
        match &mut self.super_object {
            Some(super_object) => {
                let mut super_object = super_object.borrow_mut();
                super_object.initialize(vec![], StringObject::make_object_vtable());
            }
            None => {}
        }
        if let Some(arg) = args.get(0) {
            let arg = arg.borrow();
            if let Some(arg) = arg.downcast_ref::<VectorObject>() {
                for value in arg.value.iter() {
                    self.value.push(value.borrow().downcast_ref::<PrimitiveObject<char>>().unwrap().data);
                }
            }
        }
    }
}


fn string_equals(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    let other = context.get_argument(0).unwrap();
    let other = other.borrow();
    match (object.downcast_ref::<StringObject>(), other.downcast_ref::<StringObject>()) {
        (Some(obj), Some(other)) => Ok(Some(crate::object::create_boolean(obj.value == other.value))),
        _ => Err(Fault::InvalidType(format!("String equals: Expected String")))
    }
}

fn string_to_string(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    match object.downcast_ref::<StringObject>() {
        Some(obj) => Ok(Some(crate::object::create_string(obj.value.clone()))),
        _ => Err(Fault::InvalidType(format!("String equals: Expected String")))
    }
}

fn string_order(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    let other = context.get_argument(0).unwrap();
    let other = other.borrow();
    match (object.downcast_ref::<StringObject>(), other.downcast_ref::<StringObject>()) {
        (Some(obj), Some(other)) => Ok(Some(crate::object::create_boolean(obj.value < other.value))),
        _ => Err(Fault::InvalidType(format!("String equals: Expected String")))
    }
}

fn string_length(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    match object.downcast_ref::<StringObject>() {
        Some(obj) => Ok(Some(crate::object::create_u64(obj.value.len() as u64))),
        _ => Err(Fault::InvalidType(format!("String equals: Expected String")))
    }
}

fn string_to_lowercase(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    match object.downcast_ref::<StringObject>() {
        Some(obj) => Ok(Some(crate::object::create_string(obj.value.to_lowercase()))),
        _ => Err(Fault::InvalidType(format!("String equals: Expected String")))
    }
}

fn string_to_uppercase(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    match object.downcast_ref::<StringObject>() {
        Some(obj) => Ok(Some(crate::object::create_string(obj.value.to_uppercase()))),
        _ => Err(Fault::InvalidType(format!("String equals: Expected String")))
    }
}

fn string_trim(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    match object.downcast_ref::<StringObject>() {
        Some(obj) => Ok(Some(crate::object::create_string(obj.value.trim().to_string()))),
        _ => Err(Fault::InvalidType(format!("String equals: Expected String")))
    }
}

fn string_trim_start(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    match object.downcast_ref::<StringObject>() {
        Some(obj) => Ok(Some(crate::object::create_string(obj.value.trim_start().to_string()))),
        _ => Err(Fault::InvalidType(format!("String equals: Expected String")))
    }
}

fn string_trim_end(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    match object.downcast_ref::<StringObject>() {
        Some(obj) => Ok(Some(crate::object::create_string(obj.value.trim_end().to_string()))),
        _ => Err(Fault::InvalidType(format!("String equals: Expected String")))
    }
}

fn string_contains(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    let other = context.get_argument(0).unwrap();
    let other = other.borrow();
    match (object.downcast_ref::<StringObject>(), other.downcast_ref::<StringObject>()) {
        (Some(obj), Some(other)) => Ok(Some(crate::object::create_boolean(obj.value.contains(&other.value)))),
        _ => Err(Fault::InvalidType(format!("String equals: Expected String")))
    }
}

fn string_to_vector(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    match object.downcast_ref::<StringObject>() {
        Some(obj) => {
            let vec: Vec<ObjectBox> = obj.value.chars().map(|c| crate::object::create_character(c)).collect();
            context.pop();
            Ok(Some(crate::object::create_vector(vec)))
        },
        _ => Err(Fault::InvalidType(format!("String equals: Expected String")))
    }
}

fn string_split(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    let separator = context.get_argument(0).unwrap();
    let separator = separator.borrow();
    match (object.downcast_ref::<StringObject>(), separator.downcast_ref::<StringObject>()) {
        (Some(obj), Some(separator)) => {
            let vec: Vec<ObjectBox> = obj.value.split(&separator.value).map(|s| crate::object::create_string(s.to_string())).collect();
            Ok(Some(crate::object::create_vector(vec)))
        },
        _ => Err(Fault::InvalidType(format!("String equals: Expected String")))
    }
}

fn string_get(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    let index = context.get_argument(0).unwrap();
    let index = index.borrow();
    match (object.downcast_ref::<StringObject>(), index.downcast_ref::<PrimitiveObject<u64>>()) {
        (Some(obj), Some(index)) => {
            let index = index.data as usize;
            if index < obj.value.len() {
                Ok(Some(crate::object::create_character(obj.value.chars().nth(index).unwrap())))
            } else {
                Ok(None)
            }
        },
        _ => Err(Fault::InvalidType(format!("String equals: Expected String")))
    }
}

fn string_set(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let mut object = object.borrow_mut();
    let index = context.get_argument(0).unwrap();
    let index = index.borrow();
    let value = context.get_argument(1).unwrap();
    let value = value.borrow();
    match (object.downcast_mut::<StringObject>(), index.downcast_ref::<PrimitiveObject<u64>>(), value.downcast_ref::<PrimitiveObject<char>>()) {
        (Some(obj), Some(index), Some(value)) => {
            let index = index.data as usize;
            if index < obj.value.len() {
                let mut new_string = obj.value.clone();
                new_string.replace_range(index..index+1, &value.data.to_string());
                obj.value = new_string;
                Ok(None)
            } else {
                Ok(None)
            }
        },
        _ => Err(Fault::InvalidType(format!("String equals: Expected String")))
    }
}

fn string_push(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let mut object = object.borrow_mut();
    let value = context.get_argument(0).unwrap();
    let value = value.borrow();
    match (object.downcast_mut::<StringObject>(), value.downcast_ref::<PrimitiveObject<char>>()) {
        (Some(obj), Some(value)) => {
            obj.value.push(value.data);
            Ok(None)
        },
        _ => Err(Fault::InvalidType(format!("String equals: Expected String")))
    }
}

fn string_pop(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let mut object = object.borrow_mut();
    match object.downcast_mut::<StringObject>() {
        Some(obj) => {
            if let Some(c) = obj.value.pop() {
                Ok(Some(crate::object::create_character(c)))
            } else {
                Ok(None)
            }
        },
        _ => Err(Fault::InvalidType(format!("String equals: Expected String")))
    }
}

fn string_push_char(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let mut object = object.borrow_mut();
    let value = context.get_argument(0).unwrap();
    let value = value.borrow();
    match (object.downcast_mut::<StringObject>(), value.downcast_ref::<PrimitiveObject<char>>()) {
        (Some(obj), Some(value)) => {
            obj.value.push(value.data);
            Ok(None)
        },
        _ => Err(Fault::InvalidType(format!("String equals: Expected String")))
    }
}

fn string_concat(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    let other = context.get_argument(0).unwrap();
    let other = other.borrow();
    match (object.downcast_ref::<StringObject>(), other.downcast_ref::<StringObject>()) {
        (Some(obj), Some(other)) => Ok(Some(crate::object::create_string(obj.value.clone() + &other.value))),
        _ => Err(Fault::InvalidType(format!("String equals: Expected String")))
    }
}



