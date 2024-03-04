pub mod primitive;
pub mod stack;
pub mod bytecode;
pub mod interpreter;
pub mod block;
pub mod string;

use std::sync::Arc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::hash::Hash;
use std::hash::Hasher;

use self::interpreter::Interpreter;
use self::primitive::boolean::BooleanObject;
use self::primitive::character::CharacterObject;
use self::primitive::float::{F32Object, F64Object, FloatObject};
use self::primitive::integer::{I16Object, I32Object, I64Object, I8Object, IntegerObject, U16Object, U32Object, U64Object, U8Object};
use self::primitive::{NumberObject, PrimitiveObject};
use self::string::StringObject;

pub enum Fault {
    NotImplemented,
    InvalidOperation,
    InvalidType,
    DivideByZero,
}


pub type ObjectBox<T> = Rc<RefCell<T>>;



pub trait Object: downcast_rs::Downcast {
    fn get_class(&self) -> Arc<Class>;
    fn get_super_object(&self) -> Option<ObjectBox<dyn Object>>;
    fn get_field(&self, index: usize) -> Option<ObjectBox<dyn Object>>;
    fn set_field(&mut self, index: usize, value: ObjectBox<dyn Object>);
    fn size(&self) -> Option<usize>;
    fn handle_message(&self, message: &Message) -> Option<Arc<Method>> {
        let mut method = self.get_class().get_method(&message.index);
        let class = self.get_class();
        let mut super_class = &class.super_class;
        while method.is_none() {
            if let Some(class) = super_class {
                method = class.get_method(&message.index);
                super_class = &class.super_class;
            } else {
                break;
            }
                    
        }
        method
    }
    fn process_message(&self, message: ObjectBox<dyn Object>) -> Option<Arc<Method>> {
        let message = message.borrow();
        if let Some(message) = (&*message).downcast_ref::<Message>() {
            self.handle_message(message)
        } else {
            panic!("Object::process_message: message is not a Message")
        }
    }
    fn duplicate(&self) -> ObjectBox<dyn Object>;
    
}
downcast_rs::impl_downcast!(Object);


pub struct Nil;

impl Nil {
    pub fn new() -> ObjectBox<dyn Object> {
        Rc::new(RefCell::new(Nil)) as ObjectBox<dyn Object>
    }
}

impl Object for Nil {
    fn get_class<'a>(&'a self) -> Arc<Class> {
        panic!("Nil does not have a class");
    }
    fn get_super_object(&self) -> Option<ObjectBox<dyn Object>> {
        None
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox<dyn Object>> {
        panic!("Nil does not have fields");
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox<dyn Object>) {
        panic!("Nil does not have fields");
    }
    fn size(&self) -> Option<usize> {
        panic!("Nil does not have a size");
    }
    fn handle_message(&self, _message: &Message) -> Option<Arc<Method>> {
        None
    }
    fn process_message(&self, _message: ObjectBox<dyn Object>) -> Option<Arc<Method>> {
        None
    }
    fn duplicate(&self) -> ObjectBox<dyn Object> {
        Nil::new()
    }
}

pub struct BaseObject {}

impl BaseObject {
    pub fn make_class() -> Class {
        let mut methods = HashMap::new();
        methods.insert("clone".to_string(), Arc::new(Method::RustMethod { fun: Box::new(obj_clone) }));
        methods.insert("equals".to_string(), Arc::new(Method::RustMethod { fun: Box::new(obj_equals) }));
        methods.insert("hash".to_string(), Arc::new(Method::RustMethod { fun: Box::new(obj_hash) }));
        methods.insert("to_string".to_string(), Arc::new(Method::RustMethod { fun: Box::new(obj_to_string) }));
        methods.insert("order".to_string(), Arc::new(Method::RustMethod { fun: Box::new(obj_order) }));
        Class::new(None, methods)
    }

    pub fn make_object(class: Arc<Class>) -> ObjectBox<dyn Object> {
        let object = ObjectStruct::new(class, Some(Nil::new()), 0);
        object
    }
}

fn obj_clone(object: ObjectBox<dyn Object>, _: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    let object = object.borrow();
    let new_object = object.duplicate();
    Result::Ok(Some(new_object))
}

fn obj_equals(object: ObjectBox<dyn Object>, context: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    let object_ptr = object.as_ptr();
    let other_ptr = context.arguments[0].as_ptr();
    if std::ptr::eq(object_ptr, other_ptr) {
        Ok(Some(context.create_boolean(true)))
    } else {
        Ok(Some(context.create_boolean(false)))
    }
}

fn obj_hash(object: ObjectBox<dyn Object>, context: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    let object = object.as_ptr();
    let string = format!("{:p}", object);
    let mut hasher = std::hash::DefaultHasher::new();
    string.hash(&mut hasher);
    let hash = hasher.finish();
    Ok(Some(context.create_u64(hash as u64)))
}

fn obj_to_string(object: ObjectBox<dyn Object>, context: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    let object_ptr = object.as_ptr();
    let string = format!("Object at {:p}", object_ptr);
    Ok(Some(context.create_string(string)))
}

fn obj_order(object: ObjectBox<dyn Object>, context: &mut Context, _: &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault> {
    let object_ptr = object.as_ptr();
    let other_ptr = context.arguments[0].as_ptr();
    if object_ptr as *const () < other_ptr as *const () {
        Ok(Some(context.create_i8(-1)))
    } else if object_ptr as *const () > other_ptr as *const () {
        Ok(Some(context.create_i8(1)))
    } else {
        Ok(Some(context.create_i8(0)))
    }
}



pub struct ObjectStruct {
    class: Arc<Class>,
    super_object: Option<ObjectBox<dyn Object>>,
    fields: Box<[ObjectBox<dyn Object>]>,
}

impl ObjectStruct {
    pub fn new(class: Arc<Class>, super_object: Option<ObjectBox<dyn Object>>, size: usize) -> ObjectBox<dyn Object> {
        let fields = Vec::with_capacity(size);
        ObjectBox::new(RefCell::new(ObjectStruct {
            class,
            super_object,
            fields: fields.into_boxed_slice(),
        })) as ObjectBox<dyn Object>
    }
}


impl Object for ObjectStruct {
    fn get_class(&self) -> Arc<Class> {
        self.class.clone()
    }
    fn get_super_object(&self) -> Option<ObjectBox<dyn Object>> {
        self.super_object.clone()
    }
    fn get_field(&self, index: usize) -> Option<ObjectBox<dyn Object>> {
        self.fields.get(index).map(|field| field.clone())
    }
    fn set_field(&mut self, index: usize, value: ObjectBox<dyn Object>) {
        self.fields[index] = value;
    }
    fn size(&self) -> Option<usize> {
        Some(self.fields.len())
    }
    fn duplicate(&self) -> ObjectBox<dyn Object> {
        let mut fields = Vec::with_capacity(self.fields.len());
        for field in self.fields.iter() {
            fields.push(field.clone());
        }
        let object = ObjectStruct {
            class: self.class.clone(),
            super_object: self.super_object.clone(),
            fields: fields.into_boxed_slice(),
        };
        Rc::new(RefCell::new(object)) as ObjectBox<dyn Object>
    }
}



#[derive(Clone)]
pub struct Class {
    super_class: Option<Arc<Class>>,
    methods: HashMap<String, Arc<Method>>,
}

impl Class {
    pub fn new(super_class: Option<Arc<Class>>, methods: HashMap<String, Arc<Method>>) -> Class {
        Class {
            super_class,
            methods,
        }
    }
    pub fn get_method(&self, index: &str) -> Option<Arc<Method>> {
        self.methods.get(index).cloned()
    }
}

pub enum Method {
    RustMethod {
        fun: Box<dyn Fn(ObjectBox<dyn Object>, &mut Context, &mut Interpreter) -> Result<Option<ObjectBox<dyn Object>>, Fault>>,
    },
    BytecodeMethod {
        block: ObjectBox<dyn Object>,
    },
}

impl std::fmt::Debug for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Method::RustMethod { fun: _ } => write!(f, "RustMethod"),
            Method::BytecodeMethod {..} => write!(f, "BytecodeMethod"),
        }
    }
}



pub struct Message {
    class: Arc<Class>,
    super_object: ObjectBox<dyn Object>,
    index: String,
}


impl Message {
    pub fn make_class(parent: Arc<Class>) -> Class {
        let methods = HashMap::new();
        Class::new(Some(parent), methods)
    }
    pub fn make_object(class: Arc<Class>, 
                           parent: ObjectBox<dyn Object>, 
                           index: String) -> ObjectBox<dyn Object> {
        let message = Message {
            class,
            super_object: parent,
            index,
        };
        Rc::new(RefCell::new(message)) as ObjectBox<dyn Object>
    }
}


impl Object for Message {
    fn get_class(&self) -> Arc<Class> { 
        self.class.clone()
    }
    fn get_super_object(&self) -> Option<ObjectBox<dyn Object>> {
        Some(self.super_object.clone())
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox<dyn Object>> {
        panic!("Message does not have fields");
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox<dyn Object>) {
        panic!("Message does not have fields");
    }
    fn size(&self) -> Option<usize> {
        panic!("Message does not have a size");
    }
    fn duplicate(&self) -> ObjectBox<dyn Object> {
        let message = Message {
            class: self.class.clone(),
            super_object: self.super_object.clone(),
            index: self.index.clone(),
        };
        Rc::new(RefCell::new(message)) as ObjectBox<dyn Object>
    }
}


pub struct Context {
    classes: HashMap<String, Arc<Class>>,
    pub arguments: Vec<ObjectBox<dyn Object>>,
    pub receiver: Option<ObjectBox<dyn Object>>,
}

impl Context {
    /*fn make_class(parent: &'a Class) -> Class<'a> {
        let methods = vec![];
        Class::new("Context", Some(parent), methods)
    }*/

    pub fn new() -> Context {
        let base_object_class = BaseObject::make_class();
        let classes = HashMap::new();
        let arguments = vec![];
        let mut context = Context {
            classes,
            arguments,
            receiver: None,
        };
        context.add_class("Object", base_object_class);
        let base_class = context.get_class("Object").unwrap();
        let message_class = Message::make_class(base_class.clone());
        context.add_class("Message", message_class);
        let number_class = NumberObject::make_class(base_class.clone());
        context.add_class("Number", number_class);
        let number_class = context.get_class("Number").unwrap();
        let integer_class = IntegerObject::make_class(number_class.clone());
        context.add_class("Integer", integer_class);
        let integer_class = context.get_class("Integer").unwrap();
        let i64_class = I64Object::make_class(integer_class.clone());
        context.add_class("I64", i64_class);
        let u64_class = U64Object::make_class(integer_class.clone());
        context.add_class("U64", u64_class);
        let i32_class = I32Object::make_class(integer_class.clone());
        context.add_class("I32", i32_class);
        let u32_class = U32Object::make_class(integer_class.clone());
        context.add_class("U32", u32_class);
        let i16_class = I16Object::make_class(integer_class.clone());
        context.add_class("I16", i16_class);
        let u16_class = U16Object::make_class(integer_class.clone());
        context.add_class("U16", u16_class);
        let i8_class = I8Object::make_class(integer_class.clone());
        context.add_class("I8", i8_class);
        let u8_class = U8Object::make_class(integer_class.clone());
        context.add_class("U8", u8_class);
        let float_class = FloatObject::make_class(number_class.clone());
        context.add_class("Float", float_class);
        let float_class = context.get_class("Float").unwrap();
        let f64_class = F64Object::make_class(float_class.clone());
        context.add_class("F64", f64_class);
        let f32_class = F32Object::make_class(float_class.clone());
        context.add_class("F32", f32_class);
        let string_class = StringObject::make_class(base_class.clone());
        context.add_class("String", string_class);
        let character_class = CharacterObject::make_class(base_class.clone());
        context.add_class("Char", character_class);
        let stack_class = stack::Stack::make_class(base_class.clone());
        context.add_class("Stack", stack_class);
        let block_class = block::Block::make_class(base_class.clone());
        context.add_class("Block", block_class);

        context
    }

    pub fn get_class(&self, name: &str) -> Option<Arc<Class>> {
        self.classes.get(name).cloned()
    }

    pub fn add_class(&mut self, name: &str, class: Class) {
        self.classes.insert(name.to_string(), Arc::new(class));
    }

    pub fn create_base_object(&self) -> ObjectBox<dyn Object> {
        BaseObject::make_object(self.get_class("Object").unwrap())
    }
    pub fn create_boolean(&self, value: bool) -> ObjectBox<dyn Object> {
        BooleanObject::make_object(self.get_class("Boolean").unwrap(), self.create_base_object(), value)
    }
    pub fn create_number(&self) -> ObjectBox<dyn Object> {
        NumberObject::make_object(self.get_class("Number").unwrap(), self.create_base_object())
    }
    pub fn create_integer(&self) -> ObjectBox<dyn Object> {
        IntegerObject::make_object(self.get_class("Integer").unwrap(), self.create_number())
    }
    pub fn create_i64(&self, value: i64) -> ObjectBox<dyn Object> {
        I64Object::make_object(self.get_class("I64").unwrap(), self.create_integer(), value)
    }
    pub fn create_u64(&self, value: u64) -> ObjectBox<dyn Object> {
        U64Object::make_object(self.get_class("U64").unwrap(), self.create_integer(), value)
    }
    pub fn create_i32(&self, value: i32) -> ObjectBox<dyn Object> {
        I32Object::make_object(self.get_class("I32").unwrap(), self.create_integer(), value)
    }
    pub fn create_u32(&self, value: u32) -> ObjectBox<dyn Object> {
        U32Object::make_object(self.get_class("U32").unwrap(), self.create_integer(), value)
    }
    pub fn create_i16(&self, value: i16) -> ObjectBox<dyn Object> {
        I16Object::make_object(self.get_class("I16").unwrap(), self.create_integer(), value)
    }
    pub fn create_u16(&self, value: u16) -> ObjectBox<dyn Object> {
        U16Object::make_object(self.get_class("U16").unwrap(), self.create_integer(), value)
    }
    pub fn create_i8(&self, value: i8) -> ObjectBox<dyn Object> {
        I8Object::make_object(self.get_class("I8").unwrap(), self.create_integer(), value)
    }
    pub fn create_u8(&self, value: u8) -> ObjectBox<dyn Object> {
        U8Object::make_object(self.get_class("U8").unwrap(), self.create_integer(), value)
    }
    pub fn create_float(&self) -> ObjectBox<dyn Object> {
        FloatObject::make_object(self.get_class("Float").unwrap(), self.create_number())
    }
    pub fn create_f64(&self, value: f64) -> ObjectBox<dyn Object> {
        F64Object::make_object(self.get_class("F64").unwrap(), self.create_float(), value)
    }
    pub fn create_f32(&self, value: f32) -> ObjectBox<dyn Object> {
        F32Object::make_object(self.get_class("F32").unwrap(), self.create_float(), value)
    }
    pub fn create_string(&self, value: String) -> ObjectBox<dyn Object> {
        StringObject::make_object(self.get_class("String").unwrap(), self.create_base_object(), value)
    }
    pub fn create_character(&self, value: char) -> ObjectBox<dyn Object> {
        CharacterObject::make_object(self.get_class("Char").unwrap(), self.create_base_object(), value)
    }
    pub fn create_message(&self, index: &str) -> ObjectBox<dyn Object> {
        Message::make_object(self.get_class("Message").unwrap(), self.create_base_object(), index.to_string())
    }

    pub fn attach_receiver(&mut self, receiver: ObjectBox<dyn Object>) {
        self.receiver = Some(receiver);
    }

    pub fn take_receiver(&mut self) -> Option<ObjectBox<dyn Object>> {
        self.receiver.take()
    }
}




pub fn object_clone(object: ObjectBox<dyn Object>) -> ObjectBox<dyn Object> {
    let borrowed_object = object.borrow();
    if let Some(obj) = borrowed_object.downcast_ref::<PrimitiveObject<i64>>() {
        let new_obj = obj.clone();
        return Rc::new(RefCell::new(new_obj)) as ObjectBox<dyn Object>
    } else if let Some(obj) = borrowed_object.downcast_ref::<PrimitiveObject<u64>>() {
        return Rc::new(RefCell::new(obj.clone())) as ObjectBox<dyn Object>
    } else if let Some(obj) = borrowed_object.downcast_ref::<PrimitiveObject<i32>>() {
        return Rc::new(RefCell::new(obj.clone())) as ObjectBox<dyn Object>
    } else if let Some(obj) = borrowed_object.downcast_ref::<PrimitiveObject<u32>>() {
        return Rc::new(RefCell::new(obj.clone())) as ObjectBox<dyn Object>
    } else if let Some(obj) = borrowed_object.downcast_ref::<PrimitiveObject<i16>>() {
        return Rc::new(RefCell::new(obj.clone())) as ObjectBox<dyn Object>
    }  else if let Some(obj) = borrowed_object.downcast_ref::<PrimitiveObject<u16>>() {
        return Rc::new(RefCell::new(obj.clone())) as ObjectBox<dyn Object>
    } else if let Some(obj) = borrowed_object.downcast_ref::<PrimitiveObject<i8>>() {
        return Rc::new(RefCell::new(obj.clone())) as ObjectBox<dyn Object>
    } else if let Some(obj) = borrowed_object.downcast_ref::<PrimitiveObject<u8>>() {
        return Rc::new(RefCell::new(obj.clone())) as ObjectBox<dyn Object>
    } else if let Some(obj) = borrowed_object.downcast_ref::<PrimitiveObject<bool>>() {
        return Rc::new(RefCell::new(obj.clone())) as ObjectBox<dyn Object>
    } else {
        return object.clone()
    }
}
