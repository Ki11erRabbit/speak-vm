use crate::object::Interpreter;
use super::{Class, Method, ObjectBox};
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};
use crate::object::Object;
use std::sync::Arc;
use super::Fault;
use num_traits::Zero;
use crate::create_type_ops;
use crate::object::primitive::PrimitiveObject;
use crate::object::ContextData;
use crate::object::create_boolean;

pub struct FloatObject {
    class: Arc<Class>,
    super_object: Option<ObjectBox<dyn Object>>,
}

impl FloatObject {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let mut methods = HashMap::new();
        methods.insert(String::from("is_nan"), Arc::new(Method::RustMethod { fun: Box::new(float_is_nan) }));
        methods.insert(String::from("is_infinity"), Arc::new(Method::RustMethod { fun: Box::new(float_is_infinity) }));
        methods.insert(String::from("is_neg_infinity"), Arc::new(Method::RustMethod { fun: Box::new(float_is_neg_infinity) }));
        methods.insert(String::from("is_finite"), Arc::new(Method::RustMethod { fun: Box::new(float_is_finite) }));
        methods.insert(String::from("is_normal"), Arc::new(Method::RustMethod { fun: Box::new(float_is_normal) }));
        methods.insert(String::from("floor"), Arc::new(Method::RustMethod { fun: Box::new(float_floor) }));
        methods.insert(String::from("ceil"), Arc::new(Method::RustMethod { fun: Box::new(float_ceil) }));
        methods.insert(String::from("nat_log"), Arc::new(Method::RustMethod { fun: Box::new(float_nat_log) }));
        methods.insert(String::from("log"), Arc::new(Method::RustMethod { fun: Box::new(float_log) }));
        methods.insert(String::from("hypotenuse"), Arc::new(Method::RustMethod { fun: Box::new(float_hypotenuse) }));
        methods.insert(String::from("sin"), Arc::new(Method::RustMethod { fun: Box::new(float_sin) }));
        methods.insert(String::from("cos"), Arc::new(Method::RustMethod { fun: Box::new(float_cos) }));
        methods.insert(String::from("tan"), Arc::new(Method::RustMethod { fun: Box::new(float_tan) }));
        methods.insert(String::from("arcsin"), Arc::new(Method::RustMethod { fun: Box::new(float_arcsin) }));
        methods.insert(String::from("arccos"), Arc::new(Method::RustMethod { fun: Box::new(float_arccos) }));
        methods.insert(String::from("arctan"), Arc::new(Method::RustMethod { fun: Box::new(float_arctan) }));
        
        Class::new(Some(parent), methods)
    }
    pub fn make_object(class: Arc<Class>,
                           parent: ObjectBox<dyn Object>) -> ObjectBox<dyn Object> {
        let out = Rc::new(RefCell::new(FloatObject {class, super_object: Some(parent)}));
        return out as ObjectBox<dyn Object>;
    }
}

impl Object for FloatObject {
    fn get_class(&self) -> Arc<Class> {
        self.class.clone()
    }
    fn get_super_object(&self) -> Option<ObjectBox<dyn Object>> {
        self.super_object.clone()
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox<dyn Object>> {
        panic!("Integer objects do not have fields")
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox<dyn Object>) {
        panic!("Integer objects do not have fields")
    }
    fn size(&self) -> Option<usize> {
        None
    }
    fn duplicate(&self) -> ObjectBox<dyn Object> {
        FloatObject::make_object(self.class.clone(), self.super_object.clone().unwrap())
    }
}

fn float_is_nan(_: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_is_infinity(_: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_is_neg_infinity(_: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_is_finite(_: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_is_normal(_: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_floor(_: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_ceil(_: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_nat_log(_: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_log(_: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_hypotenuse(_: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_sin(_: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_cos(_: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_tan(_: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_arcsin(_: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_arccos(_: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_arctan(_: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    Err(Fault::NotImplemented)
}

macro_rules! create_float_ops {
    ($type:ty, $is_nan:ident, $is_inf:ident, $is_neg_inf:ident, $is_finite:ident, $is_normal:ident, $floor:ident, $ceil:ident, $nat_log:ident, $log:ident, $hypotenuse:ident, $sin:ident, $cos:ident, $tan:ident, $arcsin:ident, $arccos:ident, $arctan:ident) => {
        fn $is_nan(object: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            if object.data.is_nan() {
                return Ok(Some(create_boolean(true)))
            } else {
                return Ok(Some(create_boolean(false)))
            }
        }

        fn $is_inf(object: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            if object.data.is_infinite() {
                return Ok(Some(create_boolean(true)))
            } else {
                return Ok(Some(create_boolean(false)))
            }
        }

        fn $is_neg_inf(object: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            if object.data.is_infinite() && object.data.is_sign_negative() {
                return Ok(Some(create_boolean(true)))
            } else {
                return Ok(Some(create_boolean(false)))
            }
        }

        fn $is_finite(object: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            if object.data.is_finite() {
                return Ok(Some(create_boolean(true)))
            } else {
                return Ok(Some(create_boolean(false)))
            }
        }

        fn $is_normal(object: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            if object.data.is_normal() {
                return Ok(Some(create_boolean(true)))
            } else {
                return Ok(Some(create_boolean(false)))
            }
        }

        fn $floor(object: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            object.data = object.data.floor();
            Ok(None)
        }

        fn $ceil(object: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            object.data = object.data.ceil();
            Ok(None)
        }

        fn $nat_log(object: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            object.data = object.data.ln();
            Ok(None)
        }

        fn $log(object: ObjectBox<dyn Object>, context: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            let base = context.arguments[0].borrow();
            if let Some(base) = base.downcast_ref::<PrimitiveObject<$type>>() {
                object.data = object.data.log(base.data)
            } else {
                return Err(Fault::InvalidType)
            }
            Ok(None)
        }
        
        fn $hypotenuse(object: ObjectBox<dyn Object>, context: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            let other = context.arguments[0].borrow();
            if let Some(other) = other.downcast_ref::<PrimitiveObject<$type>>() {
                object.data = object.data.hypot(other.data)
            } else {
                return Err(Fault::InvalidType)
            }
            Ok(None)
        }

        fn $sin(object: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            object.data = object.data.sin();
            Ok(None)
        }

        fn $cos(object: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            object.data = object.data.cos();
            Ok(None)
        }

        fn $tan(object: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            object.data = object.data.tan();
            Ok(None)
        }

        fn $arcsin(object: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            object.data = object.data.asin();
            Ok(None)
        }
        
        fn $arccos(object: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            object.data = object.data.acos();
            Ok(None)
        }

        fn $arctan(object: ObjectBox<dyn Object>, _: &mut ContextData, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            object.data = object.data.atan();
            Ok(None)
        }

    };
}



pub struct F64Object {
}

impl F64Object {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let mut methods = HashMap::new();
        methods.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(f64_add) }));
        methods.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(f64_sub) }));
        methods.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(f64_mul) }));
        methods.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(f64_div) }));
        methods.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(f64_mod) }));
        methods.insert(String::from("is_nan"), Arc::new(Method::RustMethod { fun: Box::new(f64_is_nan) }));
        methods.insert(String::from("is_infinity"), Arc::new(Method::RustMethod { fun: Box::new(f64_is_infinity) }));
        methods.insert(String::from("is_negitive_infinity"), Arc::new(Method::RustMethod { fun: Box::new(f64_is_neg_infinity) }));
        methods.insert(String::from("is_finite"), Arc::new(Method::RustMethod { fun: Box::new(f64_is_finite) }));
        methods.insert(String::from("is_normal"), Arc::new(Method::RustMethod { fun: Box::new(f64_is_normal) }));
        methods.insert(String::from("floor"), Arc::new(Method::RustMethod { fun: Box::new(f64_floor) }));
        methods.insert(String::from("ceil"), Arc::new(Method::RustMethod { fun: Box::new(f64_ceil) }));
        methods.insert(String::from("nat_log"), Arc::new(Method::RustMethod { fun: Box::new(f64_nat_log) }));
        methods.insert(String::from("log"), Arc::new(Method::RustMethod { fun: Box::new(f64_log) }));
        methods.insert(String::from("hypotenuse"), Arc::new(Method::RustMethod { fun: Box::new(f64_hypotenuse) }));
        methods.insert(String::from("sin"), Arc::new(Method::RustMethod { fun: Box::new(f64_sin) }));
        methods.insert(String::from("cos"), Arc::new(Method::RustMethod { fun: Box::new(f64_cos) }));
        methods.insert(String::from("tan"), Arc::new(Method::RustMethod { fun: Box::new(f64_tan) }));
        methods.insert(String::from("arcsin"), Arc::new(Method::RustMethod { fun: Box::new(f64_arcsin) }));
        methods.insert(String::from("arccos"), Arc::new(Method::RustMethod { fun: Box::new(f64_arccos) }));
        methods.insert(String::from("arctan"), Arc::new(Method::RustMethod { fun: Box::new(f64_arctan) }));

        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Arc<Class>,
                           parent: ObjectBox<dyn Object>,
                           data: f64) -> ObjectBox<dyn Object> {
        let out = Rc::new(RefCell::new(PrimitiveObject::new(class, Some(parent), data)));
        return out as ObjectBox<dyn Object>;
    }
}

create_type_ops!(f64, f64_add, f64_sub, f64_mul, f64_div, f64_mod);
create_float_ops!(f64, f64_is_nan, f64_is_infinity, f64_is_neg_infinity, f64_is_finite, f64_is_normal, f64_floor, f64_ceil, f64_nat_log, f64_log, f64_hypotenuse, f64_sin, f64_cos, f64_tan, f64_arcsin, f64_arccos, f64_arctan);

pub struct F32Object {
}

impl F32Object {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let mut methods = HashMap::new();
        methods.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(f32_add) }));
        methods.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(f32_sub) }));
        methods.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(f32_mul) }));
        methods.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(f32_div) }));
        methods.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(f32_mod) }));
        methods.insert(String::from("is_nan"), Arc::new(Method::RustMethod { fun: Box::new(f32_is_nan) }));
        methods.insert(String::from("is_infinity"), Arc::new(Method::RustMethod { fun: Box::new(f32_is_infinity) }));
        methods.insert(String::from("is_negitive_infinity"), Arc::new(Method::RustMethod { fun: Box::new(f32_is_neg_infinity) }));
        methods.insert(String::from("is_finite"), Arc::new(Method::RustMethod { fun: Box::new(f32_is_finite) }));
        methods.insert(String::from("is_normal"), Arc::new(Method::RustMethod { fun: Box::new(f32_is_normal) }));
        methods.insert(String::from("floor"), Arc::new(Method::RustMethod { fun: Box::new(f32_floor) }));
        methods.insert(String::from("ceil"), Arc::new(Method::RustMethod { fun: Box::new(f32_ceil) }));
        methods.insert(String::from("nat_log"), Arc::new(Method::RustMethod { fun: Box::new(f32_nat_log) }));
        methods.insert(String::from("log"), Arc::new(Method::RustMethod { fun: Box::new(f32_log) }));
        methods.insert(String::from("hypotenuse"), Arc::new(Method::RustMethod { fun: Box::new(f32_hypotenuse) }));
        methods.insert(String::from("sin"), Arc::new(Method::RustMethod { fun: Box::new(f32_sin) }));
        methods.insert(String::from("cos"), Arc::new(Method::RustMethod { fun: Box::new(f32_cos) }));
        methods.insert(String::from("tan"), Arc::new(Method::RustMethod { fun: Box::new(f32_tan) }));
        methods.insert(String::from("arcsin"), Arc::new(Method::RustMethod { fun: Box::new(f32_arcsin) }));
        methods.insert(String::from("arccos"), Arc::new(Method::RustMethod { fun: Box::new(f32_arccos) }));
        methods.insert(String::from("arctan"), Arc::new(Method::RustMethod { fun: Box::new(f32_arctan) }));
        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Arc<Class>,
                           parent: ObjectBox<dyn Object>,
                           data: f32) -> ObjectBox<dyn Object> {
        let out = Rc::new(RefCell::new(PrimitiveObject::new(class, Some(parent), data)));
        return out as ObjectBox<dyn Object>;
    }
}

create_type_ops!(f32, f32_add, f32_sub, f32_mul, f32_div, f32_mod);
create_float_ops!(f32, f32_is_nan, f32_is_infinity, f32_is_neg_infinity, f32_is_finite, f32_is_normal, f32_floor, f32_ceil, f32_nat_log, f32_log, f32_hypotenuse, f32_sin, f32_cos, f32_tan, f32_arcsin, f32_arccos, f32_arctan);
