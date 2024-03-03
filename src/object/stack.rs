use super::Class;
use super::Object;
use super::ObjectBox;
use std::cell::RefCell;
use std::rc::Rc;
use crate::object::Method;
use super::Context;
use std::sync::Arc;
use super::Fault;



pub struct Stack {
    class: Class,
    super_object: Option<ObjectBox<dyn Object>>,
    pub data: Vec<ObjectBox<dyn Object>>,
}

impl Stack {
    pub fn make_class(parent: Box<Class>) -> Class {
        let mut methods = Vec::new();
        methods.push(Arc::new(Method::RustMethod { fun: Box::new(stack_push) }));
        methods.push(Arc::new(Method::RustMethod { fun: Box::new(stack_pop) }));

        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Class,
                           parent: ObjectBox<dyn Object>) -> ObjectBox<dyn Object> {
        Rc::new(RefCell::new(Stack {class, super_object: Some(parent), data: Vec::new()})) as ObjectBox<dyn Object>
    }

    pub fn make_object_with_stack(class: Class,
                           parent: ObjectBox<dyn Object>,
                           data: Vec<ObjectBox<dyn Object>>) -> ObjectBox<dyn Object> {
        Rc::new(RefCell::new(Stack {class, super_object: Some(parent), data})) as ObjectBox<dyn Object>
    }
    
    pub fn push(&mut self, value: ObjectBox<dyn Object>) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<ObjectBox<dyn Object>> {
        self.data.pop()
    }

    pub fn index(&self, index: usize) -> Option<ObjectBox<dyn Object>> {
        let mut iter = self.data.iter().rev();
        for _ in 0..index {
            iter.next();
        }
        iter.next().map(|x| x.clone())
    }
}

impl Object for Stack {
    fn get_class(&self) -> &Class {
        &self.class
    }
    fn get_super_object(&self) -> Option<ObjectBox<dyn Object>> {
        self.super_object.clone()
    }
    fn get_field(&self, index: usize) -> Option<ObjectBox<dyn Object>> {
        self.index(index)
    }
    fn set_field(&mut self, index: usize, value: ObjectBox<dyn Object>) {
        let mut iter = self.data.iter_mut().rev();
        for _ in 0..index {
            iter.next();
        }
        *iter.next().unwrap() = value;
    }
    fn size(&self) -> Option<usize> {
        Some(self.data.len())
    }
}

fn stack_push(object: ObjectBox<dyn Object>, context: &mut Context) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    let mut object = object.borrow_mut();
    let object = object.downcast_mut::<Stack>();
    let value = context.arguments[0].clone();
    if let Some(object) = object {
        object.push(value);
    }
    Ok(None)
}
fn stack_pop(object: ObjectBox<dyn Object>, _: &mut Context) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    let mut object = object.borrow_mut();
    let object = object.downcast_mut::<Stack>();
    if let Some(object) = object {
        Ok(object.pop())
    } else {
        Ok(None)
    }
}

