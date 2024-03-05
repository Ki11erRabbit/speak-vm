use crate::object::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::object::primitive::PrimitiveObject;
use crate::object::ObjectBox;
use crate::object::VTable;







pub struct CharacterObject {}

impl CharacterObject {
    pub fn make_object(parent: ObjectBox, value: char) -> ObjectBox {
        Rc::new(RefCell::new(PrimitiveObject::new(Some(parent), value)))
    }
    pub fn make_vtable() -> VTable {
        let mut methods = HashMap::new();
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
        self.vtable.extend(CharacterObject::make_vtable());
        self.vtable.extend(vtable);
    }
}
       
