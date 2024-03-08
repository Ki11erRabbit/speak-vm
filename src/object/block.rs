use std::collections::HashMap;
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
    captures: Vec<ObjectBox>,
}


impl Block {
    pub fn make_object(parent: ObjectBox,
                       bytecode: Vec<ByteCode>) -> ObjectBox {
        ObjectBox::new(Block {super_object: parent, bytecode, vtable: VTable::new_empty(), captures: Vec::new()})
    }
    pub fn make_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("value"), Arc::new(Method::RustMethod { fun: Box::new(value) }));
        VTable::new(methods)
    }
    pub fn call(&self, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
        Interpreter::run_normal(&self.bytecode, context)?;
        Ok(None)
    }
}

impl Object for Block {
    fn get_vtable(&self) -> &VTable {
        &self.vtable
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        Some(self.super_object.clone())
    }
    fn get_field(&self, index: usize) -> Option<ObjectBox> {
        self.captures.get(index).cloned()
    }
    fn set_field(&mut self, index: usize, value: ObjectBox) {
        self.captures[index] = value;
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
    fn initialize(&mut self, args: Vec<ObjectBox>, vtable: VTable) {
        self.captures = args;
        self.vtable.extend(Block::make_vtable());
        self.vtable.extend(vtable);
        self.super_object.borrow_mut().initialize(vec![], VTable::new_empty());
    }
}

fn value(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    let object = object.downcast_ref::<Block>().expect("Expected block");
    Interpreter::run_normal(&object.bytecode, context)?;
    Ok(None)
}
