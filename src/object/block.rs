use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::object::{Object, ObjectBox};
use crate::object::Fault;
use crate::vm::interpreter::Interpreter;
use crate::object::Method;
use std::sync::Arc;

use crate::vm::bytecode::ByteCode;
use super::{ContextData, VTable};







pub struct Block {
    super_object: ObjectBox,
    vtable: VTable,
    pub bytecode: Vec<ByteCode>,
}


impl Block {
    pub fn make_object(parent: ObjectBox,
                       bytecode: Vec<ByteCode>) -> ObjectBox {
        Rc::new(RefCell::new(Block {super_object: parent, bytecode, vtable: VTable::new_empty()})) as ObjectBox
    }
    pub fn make_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("value"), Arc::new(Method::RustMethod { fun: Box::new(value) }));
        VTable::new(methods)
    }
}

impl Object for Block {
    fn get_vtable(&self) -> &VTable {
        &self.vtable
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        Some(self.super_object.clone())
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox> {
        panic!("Block objects do not have fields")
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox) {
        panic!("Block objects do not have fields")
    }
    fn size(&self) -> Option<usize> {
        None
    }
    fn duplicate(&self) -> ObjectBox {
        let block = Block::make_object(self.super_object.borrow().duplicate(), self.bytecode.clone());
        let mut blk = block.borrow_mut();
        blk.initialize(Vec::new(), self.vtable.clone());
        drop(blk);
        block
    }
    fn initialize(&mut self, _: Vec<ObjectBox>, vtable: VTable) {
        self.vtable.extend(Block::make_vtable());
        self.vtable.extend(vtable);
        self.super_object.borrow_mut().initialize(vec![], VTable::new_empty());
    }
}

fn value(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    let object = object.downcast_ref::<Block>().expect("Expected block");
    for code in object.bytecode.iter() {
        Interpreter::run(context, code.clone());
    }
    Ok(None)
}
