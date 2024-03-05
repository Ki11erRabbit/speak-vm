use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{Object, ObjectBox, VTable};





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
        Rc::new(RefCell::new(string)) as ObjectBox
    }
    pub fn make_vtable() -> VTable {
        let mut methods = HashMap::new();
        //methods.insert(String::from("length"), Arc::new(Method::RustMethod { fun: Box::new(string_length) }));
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
    fn initialize(&mut self, _: Vec<ObjectBox>, vtable: VTable) {
        self.vtable.extend(StringObject::make_vtable());
        self.vtable.extend(vtable);
    }
}










