use super::ContextData;
use super::Object;
use super::ObjectBox;
use super::VTable;
use std::collections::HashMap;
use crate::object::Method;
use std::sync::Arc;
use super::Fault;



pub struct Stack {
    super_object: Option<ObjectBox>,
    vtable: VTable,
    pub data: Vec<ObjectBox>,
}

impl Stack {
    pub fn make_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("push"), Arc::new(Method::RustMethod { fun: Box::new(stack_push) }));
        methods.insert(String::from("pop"), Arc::new(Method::RustMethod { fun: Box::new(stack_pop) }));
        VTable::new(methods)
    }

    pub fn make_object(parent: ObjectBox) -> ObjectBox {
        ObjectBox::new(Stack {super_object: Some(parent), data: Vec::new(), vtable: VTable::new_empty()})
    }

    pub fn make_object_with_stack(parent: ObjectBox,
                                  data: Vec<ObjectBox>) -> ObjectBox {
        ObjectBox::new(Stack {super_object: Some(parent), data, vtable: VTable::new_empty()})
    }
    
    pub fn push(&mut self, value: ObjectBox) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<ObjectBox> {
        self.data.pop()
    }

    pub fn index(&self, index: usize) -> Option<ObjectBox> {
        let mut iter = self.data.iter().rev();
        for _ in 0..index {
            iter.next();
        }
        iter.next().map(|x| x.clone())
    }
}

impl Object for Stack {
    fn get_vtable(&self) -> &VTable {
        &self.vtable
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        self.super_object.clone()
    }
    fn get_field(&self, index: usize) -> Option<ObjectBox> {
        self.index(index)
    }
    fn set_field(&mut self, index: usize, value: ObjectBox) {
        let mut iter = self.data.iter_mut().rev();
        for _ in 0..index {
            iter.next();
        }
        *iter.next().unwrap() = value;
    }
    fn size(&self) -> Option<usize> {
        Some(self.data.len())
    }
    fn duplicate(&self) -> ObjectBox {
        let stack = Stack::make_object_with_stack(self.super_object.clone().unwrap().borrow().duplicate(), self.data.clone());
        let mut stk = stack.borrow_mut();
        stk.initialize(Vec::new(), self.vtable.clone());
        drop(stk);
        stack
    }
    fn initialize(&mut self, arguments: Vec<ObjectBox>, vtable: VTable) {
        for arg in arguments {
            self.push(arg);
        }
        self.vtable.extend(Stack::make_vtable());
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

fn stack_push(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let mut object = object.borrow_mut();
    let object = object.downcast_mut::<Stack>();
    let value = context.arguments[0].clone();
    if let Some(object) = object {
        object.push(value);
    }
    Ok(None)
}
fn stack_pop(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let mut object = object.borrow_mut();
    let object = object.downcast_mut::<Stack>();
    if let Some(object) = object {
        Ok(object.pop())
    } else {
        Ok(None)
    }
}

