use super::{Class, Method, ObjectBox};
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};
use crate::object::{Object, VTable};
use std::sync::Arc;
use super::Fault;
use num_traits::Zero;
use crate::create_type_ops;
use crate::object::primitive::PrimitiveObject;
use crate::object::ContextData;
use crate::object::create_boolean;

pub struct FloatObject {
    class: Arc<Class>,
    super_object: Option<ObjectBox>,
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
                           parent: ObjectBox) -> ObjectBox {
        let out = Rc::new(RefCell::new(FloatObject {class, super_object: Some(parent)}));
        return out as ObjectBox;
    }
}

impl Object for FloatObject {
    fn get_class(&self) -> Arc<Class> {
        self.class.clone()
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        self.super_object.clone()
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox> {
        panic!("Integer objects do not have fields")
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox) {
        panic!("Integer objects do not have fields")
    }
    fn size(&self) -> Option<usize> {
        None
    }
    fn duplicate(&self) -> ObjectBox {
        FloatObject::make_object(self.class.clone(), self.super_object.clone().unwrap())
    }
    fn initalize(&mut self, _: Vec<ObjectBox>) {
        panic!("Integer objects do not take arguments");
    }
}

fn float_is_nan(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_is_infinity(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_is_neg_infinity(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_is_finite(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_is_normal(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_floor(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_ceil(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_nat_log(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_log(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_hypotenuse(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_sin(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_cos(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_tan(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_arcsin(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_arccos(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

fn float_arctan(_: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    Err(Fault::NotImplemented)
}

macro_rules! create_float_ops {
    ($type:ty, $is_nan:ident, $is_inf:ident, $is_neg_inf:ident, $is_finite:ident, $is_normal:ident, $floor:ident, $ceil:ident, $nat_log:ident, $log:ident, $hypotenuse:ident, $sin:ident, $cos:ident, $tan:ident, $arcsin:ident, $arccos:ident, $arctan:ident) => {
        fn $is_nan(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            if object.data.is_nan() {
                return Ok(Some(create_boolean(true)))
            } else {
                return Ok(Some(create_boolean(false)))
            }
        }

        fn $is_inf(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            if object.data.is_infinite() {
                return Ok(Some(create_boolean(true)))
            } else {
                return Ok(Some(create_boolean(false)))
            }
        }

        fn $is_neg_inf(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            if object.data.is_infinite() && object.data.is_sign_negative() {
                return Ok(Some(create_boolean(true)))
            } else {
                return Ok(Some(create_boolean(false)))
            }
        }

        fn $is_finite(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            if object.data.is_finite() {
                return Ok(Some(create_boolean(true)))
            } else {
                return Ok(Some(create_boolean(false)))
            }
        }

        fn $is_normal(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            if object.data.is_normal() {
                return Ok(Some(create_boolean(true)))
            } else {
                return Ok(Some(create_boolean(false)))
            }
        }

        fn $floor(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            object.data = object.data.floor();
            Ok(None)
        }

        fn $ceil(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            object.data = object.data.ceil();
            Ok(None)
        }

        fn $nat_log(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            object.data = object.data.ln();
            Ok(None)
        }

        fn $log(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
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
        
        fn $hypotenuse(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
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

        fn $sin(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            object.data = object.data.sin();
            Ok(None)
        }

        fn $cos(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            object.data = object.data.cos();
            Ok(None)
        }

        fn $tan(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            object.data = object.data.tan();
            Ok(None)
        }

        fn $arcsin(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            object.data = object.data.asin();
            Ok(None)
        }
        
        fn $arccos(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
            let mut object =  object.borrow_mut();
            let object = object.downcast_mut::<PrimitiveObject<$type>>().ok_or(Fault::InvalidType)?;
            object.data = object.data.acos();
            Ok(None)
        }

        fn $arctan(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
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

        
        Class::new(Some(parent), methods)
    }

    pub fn make_object(class: Arc<Class>,
                           parent: ObjectBox,
                           data: f64) -> ObjectBox {
        let out = Rc::new(RefCell::new(PrimitiveObject::new(class, Some(parent), data)));
        return out as ObjectBox;
    }
}


impl Object for PrimitiveObject<f64> {
    fn get_class(&self) -> Arc<Class> {
        self.class.clone()
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        self.super_object.clone()
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox> {
        panic!("Float objects do not have fields")
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox) {
        panic!("Float objects do not have fields")
    }
    fn size(&self) -> Option<usize> {
        None
    }
    fn duplicate(&self) -> ObjectBox {
        F64Object::make_object(self.class.clone(), self.super_object.clone().unwrap(), self.data)
    }
    fn initialize(&mut self, _: Vec<ObjectBox>, vtable: VTable) {
        let mut number_vtable = HashMap::new();
        number_vtable.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(f64_add) }));
        number_vtable.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(f64_sub) }));
        number_vtable.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(f64_mul) }));
        number_vtable.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(f64_div) }));
        number_vtable.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(f64_mod) }));
        let number_vtable = VTable::new(number_vtable);

        let mut float_vtable = HashMap::new();
        float_vtable.insert(String::from("is_nan"), Arc::new(Method::RustMethod { fun: Box::new(f64_is_nan) }));
        float_vtable.insert(String::from("is_infinity"), Arc::new(Method::RustMethod { fun: Box::new(f64_is_infinity) }));
        float_vtable.insert(String::from("is_negitive_infinity"), Arc::new(Method::RustMethod { fun: Box::new(f64_is_neg_infinity) }));
        float_vtable.insert(String::from("is_finite"), Arc::new(Method::RustMethod { fun: Box::new(f64_is_finite) }));
        float_vtable.insert(String::from("is_normal"), Arc::new(Method::RustMethod { fun: Box::new(f64_is_normal) }));
        float_vtable.insert(String::from("floor"), Arc::new(Method::RustMethod { fun: Box::new(f64_floor) }));
        float_vtable.insert(String::from("ceil"), Arc::new(Method::RustMethod { fun: Box::new(f64_ceil) }));
        float_vtable.insert(String::from("nat_log"), Arc::new(Method::RustMethod { fun: Box::new(f64_nat_log) }));
        float_vtable.insert(String::from("log"), Arc::new(Method::RustMethod { fun: Box::new(f64_log) }));
        float_vtable.insert(String::from("hypotenuse"), Arc::new(Method::RustMethod { fun: Box::new(f64_hypotenuse) }));
        float_vtable.insert(String::from("sin"), Arc::new(Method::RustMethod { fun: Box::new(f64_sin) }));
        float_vtable.insert(String::from("cos"), Arc::new(Method::RustMethod { fun: Box::new(f64_cos) }));
        float_vtable.insert(String::from("tan"), Arc::new(Method::RustMethod { fun: Box::new(f64_tan) }));
        float_vtable.insert(String::from("arcsin"), Arc::new(Method::RustMethod { fun: Box::new(f64_arcsin) }));
        float_vtable.insert(String::from("arccos"), Arc::new(Method::RustMethod { fun: Box::new(f64_arccos) }));
        float_vtable.insert(String::from("arctan"), Arc::new(Method::RustMethod { fun: Box::new(f64_arctan) }));
        let float_vtable = VTable::new(float_vtable);

        let float_object = self.get_super_object().unwrap().clone();
        let mut float_object = float_object.borrow_mut();
        float_object.initialize(Vec::new(), float_vtable);
        let number_object = float_object.get_super_object().unwrap().clone();
        let mut number_object = number_object.borrow_mut();
        number_object.initialize(Vec::new(), number_vtable);
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
                           parent: ObjectBox,
                           data: f32) -> ObjectBox {
        let out = Rc::new(RefCell::new(PrimitiveObject::new(class, Some(parent), data)));
        return out as ObjectBox;
    }
}

impl Object for PrimitiveObject<f32> {
    fn get_class(&self) -> Arc<Class> {
        self.class.clone()
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        self.super_object.clone()
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox> {
        panic!("Float objects do not have fields")
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox) {
        panic!("Float objects do not have fields")
    }
    fn size(&self) -> Option<usize> {
        None
    }
    fn duplicate(&self) -> ObjectBox {
        F32Object::make_object(self.class.clone(), self.super_object.clone().unwrap(), self.data)
    }
    fn initialize(&mut self, _: Vec<ObjectBox>, vtable: VTable) {
        let mut number_vtable = HashMap::new();
        number_vtable.insert(String::from("add"), Arc::new(Method::RustMethod { fun: Box::new(f32_add) }));
        number_vtable.insert(String::from("sub"), Arc::new(Method::RustMethod { fun: Box::new(f32_sub) }));
        number_vtable.insert(String::from("mul"), Arc::new(Method::RustMethod { fun: Box::new(f32_mul) }));
        number_vtable.insert(String::from("div"), Arc::new(Method::RustMethod { fun: Box::new(f32_div) }));
        number_vtable.insert(String::from("mod"), Arc::new(Method::RustMethod { fun: Box::new(f32_mod) }));
        let number_vtable = VTable::new(number_vtable);

        let mut float_vtable = HashMap::new();
        float_vtable.insert(String::from("is_nan"), Arc::new(Method::RustMethod { fun: Box::new(f32_is_nan) }));
        float_vtable.insert(String::from("is_infinity"), Arc::new(Method::RustMethod { fun: Box::new(f32_is_infinity) }));
        float_vtable.insert(String::from("is_negitive_infinity"), Arc::new(Method::RustMethod { fun: Box::new(f32_is_neg_infinity) }));
        float_vtable.insert(String::from("is_finite"), Arc::new(Method::RustMethod { fun: Box::new(f32_is_finite) }));
        float_vtable.insert(String::from("is_normal"), Arc::new(Method::RustMethod { fun: Box::new(f32_is_normal) }));
        float_vtable.insert(String::from("floor"), Arc::new(Method::RustMethod { fun: Box::new(f32_floor) }));
        float_vtable.insert(String::from("ceil"), Arc::new(Method::RustMethod { fun: Box::new(f32_ceil) }));
        float_vtable.insert(String::from("nat_log"), Arc::new(Method::RustMethod { fun: Box::new(f32_nat_log) }));
        float_vtable.insert(String::from("log"), Arc::new(Method::RustMethod { fun: Box::new(f32_log) }));
        float_vtable.insert(String::from("hypotenuse"), Arc::new(Method::RustMethod { fun: Box::new(f32_hypotenuse) }));
        float_vtable.insert(String::from("sin"), Arc::new(Method::RustMethod { fun: Box::new(f32_sin) }));
        float_vtable.insert(String::from("cos"), Arc::new(Method::RustMethod { fun: Box::new(f32_cos) }));
        float_vtable.insert(String::from("tan"), Arc::new(Method::RustMethod { fun: Box::new(f32_tan) }));
        float_vtable.insert(String::from("arcsin"), Arc::new(Method::RustMethod { fun: Box::new(f32_arcsin) }));
        float_vtable.insert(String::from("arccos"), Arc::new(Method::RustMethod { fun: Box::new(f32_arccos) }));
        float_vtable.insert(String::from("arctan"), Arc::new(Method::RustMethod { fun: Box::new(f32_arctan) }));
        let float_vtable = VTable::new(float_vtable);

        let float_object = self.get_super_object().unwrap().clone();
        let mut float_object = float_object.borrow_mut();
        float_object.initialize(Vec::new(), float_vtable);
        let number_object = float_object.get_super_object().unwrap().clone();
        let mut number_object = number_object.borrow_mut();
        number_object.initialize(Vec::new(), number_vtable);
    }
}


create_type_ops!(f32, f32_add, f32_sub, f32_mul, f32_div, f32_mod);
create_float_ops!(f32, f32_is_nan, f32_is_infinity, f32_is_neg_infinity, f32_is_finite, f32_is_normal, f32_floor, f32_ceil, f32_nat_log, f32_log, f32_hypotenuse, f32_sin, f32_cos, f32_tan, f32_arcsin, f32_arccos, f32_arctan);
