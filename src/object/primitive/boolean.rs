use std::sync::Arc;
use std::collections::HashMap;
use crate::object::{Object, ObjectBox};
use crate::object::VTable;
use super::PrimitiveObject;
use crate::object::ContextData;
use crate::object::Fault;
use crate::object::Method;








pub struct BooleanObject {}


impl BooleanObject {
    pub fn make_object(parent: ObjectBox, data: bool) -> ObjectBox {
        ObjectBox::new(PrimitiveObject::new(Some(parent), data))
    }
    pub fn make_object_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert("equals".to_string(), Arc::new(Method::RustMethod { fun: Box::new(boolean_equals) }));
        methods.insert("to_string".to_string(), Arc::new(Method::RustMethod { fun: Box::new(boolean_to_string) }));
        methods.insert("order".to_string(), Arc::new(Method::RustMethod { fun: Box::new(boolean_order) }));
        VTable::new(methods)
    }
    pub fn make_vtable() -> VTable {
        let methods = HashMap::new();
        VTable::new(methods)
    }
}

impl Object for PrimitiveObject<bool> {
    fn get_vtable(&self) -> &VTable {
        &self.vtable
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        self.super_object.clone()
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox> {
        panic!("Boolean objects do not have fields")
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox) {
        panic!("Boolean objects do not have fields")
    }
    fn size(&self) -> Option<usize> {
        None
    }
    fn duplicate(&self) -> ObjectBox {
        let boolean = BooleanObject::make_object(self.super_object.clone().unwrap().borrow().duplicate(), self.data);
        let mut bln = boolean.borrow_mut();
        bln.initialize(Vec::new(), self.vtable.clone());
        drop(bln);
        boolean as ObjectBox
    }
    fn initialize(&mut self, _: Vec<ObjectBox>, vtable: VTable) {
        let bool_vtable = BooleanObject::make_vtable();
        let object_vtable = BooleanObject::make_object_vtable();
        let parent = self.super_object.clone().unwrap();
        let mut parent = parent.borrow_mut();
        parent.initialize(Vec::new(), object_vtable);
        self.vtable.extend(bool_vtable);
        self.vtable.extend(vtable);
    }
}



fn boolean_equals(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    let other = context.get_argument(0).unwrap();
    let other = other.borrow();
    match (object.downcast_ref::<PrimitiveObject<bool>>(), other.downcast_ref::<PrimitiveObject<bool>>()) {
        (Some(obj), Some(other)) => Ok(Some(crate::object::create_boolean(obj.data == other.data))),
        _ => Err(Fault::InvalidType)
    }
}

fn boolean_to_string(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    match object.downcast_ref::<PrimitiveObject<bool>>() {
        Some(obj) => Ok(Some(crate::object::create_string(obj.data.to_string()))),
        _ => Err(Fault::InvalidType)
    }
}

fn boolean_order(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    let other = context.get_argument(0).unwrap();
    let other = other.borrow();
    match (object.downcast_ref::<PrimitiveObject<bool>>(), other.downcast_ref::<PrimitiveObject<bool>>()) {
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







