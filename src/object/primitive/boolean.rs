use std::cell::RefCell;
use std::{collections::HashMap, rc::Rc};
use crate::object::{Object, ObjectBox};
use crate::object::VTable;
use super::PrimitiveObject;








pub struct BooleanObject {}


impl BooleanObject {
    pub fn make_object(parent: ObjectBox, data: bool) -> ObjectBox {
        Rc::new(RefCell::new(PrimitiveObject::new(Some(parent), data)))
    }
    pub fn make_vtable() -> VTable {
        let mut methods = HashMap::new();
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
        self.vtable.extend(vtable);
    }
}








