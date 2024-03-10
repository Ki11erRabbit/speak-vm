use std::collections::HashMap;
use std::sync::Arc;
use super::{block::Block, ContextData, Fault, Object, ObjectBox, PrimitiveObject, VTable, Method};





pub struct VectorObject {
    super_object: Option<ObjectBox>,
    vtable: VTable,
    pub value: Box<[ObjectBox]>,
}


impl VectorObject {
    pub fn make_object(parent: ObjectBox, value: Box<[ObjectBox]>) -> ObjectBox {
        let vector = VectorObject {
            super_object: Some(parent),
            value,
            vtable: VTable::new_empty(),
        };
        ObjectBox::new(vector)
    }
    pub fn make_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert(String::from("length"), Arc::new(Method::RustMethod { fun: Box::new(vector_length) }));
        methods.insert(String::from("get"), Arc::new(Method::RustMethod { fun: Box::new(vector_get) }));
        methods.insert(String::from("set"), Arc::new(Method::RustMethod { fun: Box::new(vector_set) }));
        methods.insert(String::from("map"), Arc::new(Method::RustMethod { fun: Box::new(vector_map) }));
        methods.insert(String::from("fold"), Arc::new(Method::RustMethod { fun: Box::new(vector_fold) }));
        methods.insert(String::from("sort"), Arc::new(Method::RustMethod { fun: Box::new(vector_sort) }));
        methods.insert(String::from("concat"), Arc::new(Method::RustMethod { fun: Box::new(vector_concat) }));
        VTable::new(methods)
    }
}


impl Object for VectorObject {
    fn get_vtable(&self) -> &VTable {
        &self.vtable
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        self.super_object.clone()
    }
    fn get_field(&self, index: usize) -> Option<ObjectBox> {
        Some(self.value[index].clone())
    }
    fn set_field(&mut self, index: usize, value: ObjectBox) {
        self.value[index] = value;
    }
    fn size(&self) -> Option<usize> {
        Some(self.value.len())
    }
    fn duplicate(&self) -> ObjectBox {
        let vector = VectorObject::make_object(self.super_object.clone().unwrap().borrow().duplicate(), self.value.clone());
        let mut vec_obj = vector.borrow_mut();
        vec_obj.initialize(Vec::new(), self.vtable.clone());
        drop(vec_obj);
        vector
    }
    fn initialize(&mut self, args: Vec<ObjectBox>, vtable: VTable) {
        self.vtable.extend(VectorObject::make_vtable());
        self.vtable.extend(vtable);
        match &mut self.super_object {
            Some(super_object) => {
                super_object.borrow_mut().initialize(Vec::new(), self.vtable.clone());
            }
            None => {}
        }
        let arg = args.get(0).unwrap();
        let arg = arg.borrow();
        let arg = arg.downcast_ref::<PrimitiveObject<u64>>().unwrap();
        let size = arg.data as usize;
        let mut vec = Vec::new();
        vec.resize(size, super::Nil::new());
        self.value = vec.into_boxed_slice();
    }
}


fn vector_get(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let index = context.arguments[0].clone();
    let index = index.borrow();
    let index = index.downcast_ref::<PrimitiveObject<u64>>().ok_or(Fault::InvalidType(format!("Vector get: Expected u64")))?;
    let index = index.data as usize;
    let vector = object.borrow();
    let vector = vector.downcast_ref::<VectorObject>().ok_or(Fault::InvalidType(format!("Vector get: Expected Vector")))?;
    Ok(Some(vector.get_field(index).unwrap()))
}

fn vector_set(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let index = context.arguments[0].clone();
    let index = index.borrow();
    let index = index.downcast_ref::<PrimitiveObject<u64>>().ok_or(Fault::InvalidType(format!("Vector set: Expected u64")))?;
    let index = index.data as usize;
    let value = context.arguments[1].clone();
    let mut vector = object.borrow_mut();
    let vector = vector.downcast_mut::<VectorObject>().ok_or(Fault::InvalidType(format!("Vector set: Expected Vector")))?;
    vector.set_field(index, value);
    Ok(None)
}

fn vector_length(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let vector = object.borrow();
    let vector = vector.downcast_ref::<VectorObject>().ok_or(Fault::InvalidType(format!("Vector length: Expected Vector")))?;
    Ok(Some(super::create_u64(vector.size().unwrap() as u64)))
}

fn vector_map(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let function = context.arguments[0].clone();
    let function = function.borrow();
    let function = function.downcast_ref::<Block>().ok_or(Fault::InvalidType(format!("Vector map: Expected Block")))?;
    let vector = object.borrow();
    let vector = vector.downcast_ref::<VectorObject>().ok_or(Fault::InvalidType(format!("Vector map: Expected map")))?;
    let mut new_vector = Vec::new();
    for item in vector.value.iter() {
        context.set_argument(0, item.clone());
        context.arg_count = 1;
        let result = function.call(context)?;
        if let Some(result) = result {
            new_vector.push(result);
        }
    }
    Ok(Some(VectorObject::make_object(object.clone(), new_vector.into_boxed_slice())))
}

fn vector_fold(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let function = context.arguments[0].clone();
    let function = function.borrow();
    let function = function.downcast_ref::<Block>().ok_or(Fault::InvalidType(format!("Vector fold: Expected Block")))?;
    let vector = object.borrow();
    let vector = vector.downcast_ref::<VectorObject>().ok_or(Fault::InvalidType(format!("Vector fold: Expected Vector")))?;
    let mut result = context.arguments[1].clone();
    for item in vector.value.iter() {
        context.set_argument(0, result.clone());
        context.set_argument(1, item.clone());
        context.arg_count = 2;
        result = function.call(context)?.unwrap();
    }
    Ok(Some(result))
}

fn vector_sort(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let mut vector = object.borrow_mut();
    let vector = vector.downcast_mut::<VectorObject>().ok_or(Fault::InvalidType(format!("Vector sort: Expected Vector")))?;
    vector.value.sort_by(|a, b| {
        let a_ = a.borrow();
        let method = a_.process_message(super::create_message("order")).expect("Expected order method");
        context.set_argument(0, b.clone());
        context.arg_count = 1;
        match method.call(a.clone(), context) {
            Ok(Some(result)) => {
                let result = result.borrow();
                let result = result.downcast_ref::<PrimitiveObject<i8>>().expect("Expected integer");
                if result.data > 0 {
                    std::cmp::Ordering::Greater
                } else if result.data < 0 {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            }
            Ok(None) => {
                panic!("Order method returned nothing");
            }
            Err(fault) => {
                panic!("Order method failed: {:?}", fault);
            }
        }
    });
    Ok(None)
}

fn vector_concat(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let vector = object.borrow();
    let vector = vector.downcast_ref::<VectorObject>().ok_or(Fault::InvalidType(format!("Vector concat: Expected Vector")))?;
    let mut new_vector = Vec::new();
    for item in vector.value.iter() {
        new_vector.push(item.clone());
    }
    for item in context.arguments.iter() {
        let item = item.borrow();
        let item = item.downcast_ref::<VectorObject>().ok_or(Fault::InvalidType(format!("Vector concat: Expected Vector")))?;
        for item in item.value.iter() {
            new_vector.push(item.clone());
        }
    }
    context.pop();
    Ok(Some(VectorObject::make_object(object.clone(), new_vector.into_boxed_slice())))
}
