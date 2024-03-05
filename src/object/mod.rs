pub mod primitive;
pub mod stack;
pub mod block;
pub mod string;
pub mod log;

use lazy_static::lazy_static;
use std::sync::Arc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::hash::Hash;
use std::hash::Hasher;
use std::sync::RwLock;

use self::log::Logger;
use self::primitive::boolean::BooleanObject;
use self::primitive::character::CharacterObject;
use self::primitive::float::{F32Object, F64Object, FloatObject};
use self::primitive::integer::{I16Object, I32Object, I64Object, I8Object, IntegerObject, U16Object, U32Object, U64Object, U8Object};
use self::primitive::{NumberObject, PrimitiveObject};
use self::string::StringObject;

#[derive(Debug)]
pub enum Fault {
    NotImplemented,
    InvalidOperation,
    InvalidType,
    DivideByZero,
    IO(std::io::Error),
}


pub type ObjectBox = Rc<RefCell<dyn Object>>;



pub trait Object: downcast_rs::Downcast {
    //fn get_class(&self) -> Arc<Class>;
    fn get_vtable(&self) -> &VTable;
    fn get_super_object(&self) -> Option<ObjectBox>;
    fn get_field(&self, index: usize) -> Option<ObjectBox>;
    fn set_field(&mut self, index: usize, value: ObjectBox);
    fn size(&self) -> Option<usize>;
    fn handle_message(&self, message: &Message) -> Option<Arc<Method>> {
        let mut method = self.get_vtable().get_method(&message.index);
        let mut object = self.get_super_object();
        while method.is_none() {
            if let Some(obj) = object {
                let obj = obj.borrow();
                method = obj.get_vtable().get_method(&message.index);
                object = obj.get_super_object();
            } else {
                break;
            }
                    
        }
        method
    }
    fn process_message(&self, message: ObjectBox) -> Option<Arc<Method>> {
        let message = message.borrow();
        if let Some(message) = (&*message).downcast_ref::<Message>() {
            self.handle_message(message)
        } else {
            panic!("Object::process_message: message is not a Message")
        }
    }
    fn duplicate(&self) -> ObjectBox;
    fn initialize(&mut self, arguments: Vec<ObjectBox>, vtable: VTable);
    
}
downcast_rs::impl_downcast!(Object);


pub struct Nil;

impl Nil {
    pub fn new() -> ObjectBox {
        Rc::new(RefCell::new(Nil)) as ObjectBox
    }
}

impl Object for Nil {
    /*fn get_class<'a>(&'a self) -> Arc<Class> {
        panic!("Nil does not have a class");
    }*/
    fn get_vtable(&self) -> &VTable {
        panic!("Nil does not have a vtable");
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        None
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox> {
        panic!("Nil does not have fields");
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox) {
        panic!("Nil does not have fields");
    }
    fn size(&self) -> Option<usize> {
        panic!("Nil does not have a size");
    }
    fn handle_message(&self, _message: &Message) -> Option<Arc<Method>> {
        None
    }
    fn process_message(&self, _message: ObjectBox) -> Option<Arc<Method>> {
        None
    }
    fn duplicate(&self) -> ObjectBox {
        Nil::new()
    }
    fn initialize(&mut self, _arguments: Vec<ObjectBox>, _: VTable) {
        panic!("Nil does not have fields");
    }
}

pub struct BaseObject {
    vtable: VTable,
}

impl BaseObject {
    /*pub fn make_class() -> Class {
        let mut methods = HashMap::new();
        methods.insert("clone".to_string(), Arc::new(Method::RustMethod { fun: Box::new(obj_clone) }));
        methods.insert("equals".to_string(), Arc::new(Method::RustMethod { fun: Box::new(obj_equals) }));
        methods.insert("hash".to_string(), Arc::new(Method::RustMethod { fun: Box::new(obj_hash) }));
        methods.insert("to_string".to_string(), Arc::new(Method::RustMethod { fun: Box::new(obj_to_string) }));
        methods.insert("order".to_string(), Arc::new(Method::RustMethod { fun: Box::new(obj_order) }));
        methods.insert("initalize".to_string(), Arc::new(Method::RustMethod { fun: Box::new(obj_initalize) }));
        Class::new(None, methods)
    }*/
    pub fn make_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert("clone".to_string(), Arc::new(Method::RustMethod { fun: Box::new(obj_clone) }));
        methods.insert("equals".to_string(), Arc::new(Method::RustMethod { fun: Box::new(obj_equals) }));
        methods.insert("hash".to_string(), Arc::new(Method::RustMethod { fun: Box::new(obj_hash) }));
        methods.insert("to_string".to_string(), Arc::new(Method::RustMethod { fun: Box::new(obj_to_string) }));
        methods.insert("order".to_string(), Arc::new(Method::RustMethod { fun: Box::new(obj_order) }));
        methods.insert("initalize".to_string(), Arc::new(Method::RustMethod { fun: Box::new(obj_initalize) }));
        VTable::new(methods)
    }

    pub fn make_object() -> ObjectBox {
        Rc::new(RefCell::new(BaseObject {vtable: VTable::new_empty()})) as ObjectBox
    }
}

impl Object for BaseObject {
    fn get_vtable(&self) -> &VTable {
        &self.vtable
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        None
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox> {
        panic!("BaseObject does not have fields");
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox) {
        panic!("BaseObject does not have fields");
    }
    fn size(&self) -> Option<usize> {
        panic!("BaseObject does not have a size");
    }
    fn handle_message(&self, _message: &Message) -> Option<Arc<Method>> {
        None
    }
    fn process_message(&self, _message: ObjectBox) -> Option<Arc<Method>> {
        None
    }
    fn duplicate(&self) -> ObjectBox {
        BaseObject::make_object()
    }
    fn initialize(&mut self, _arguments: Vec<ObjectBox>, vtable: VTable) {
        self.vtable.extend(BaseObject::make_vtable());
        self.vtable.extend(vtable);
    }
}

fn obj_clone(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.borrow();
    let new_object = object.duplicate();
    Result::Ok(Some(new_object))
}

fn obj_equals(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object_ptr = object.as_ptr();
    let other_ptr = context.arguments[0].as_ptr();
    if std::ptr::eq(object_ptr, other_ptr) {
        Ok(Some(create_boolean(true)))
    } else {
        Ok(Some(create_boolean(false)))
    }
}

fn obj_hash(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.as_ptr();
    let string = format!("{:p}", object);
    let mut hasher = std::hash::DefaultHasher::new();
    string.hash(&mut hasher);
    let hash = hasher.finish();
    Ok(Some(create_u64(hash as u64)))
}

fn obj_to_string(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object_ptr = object.as_ptr();
    let string = format!("Object at {:p}", object_ptr);
    Ok(Some(create_string(string)))
}

fn obj_order(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object_ptr = object.as_ptr();
    let other_ptr = context.arguments[0].as_ptr();
    if object_ptr as *const () < other_ptr as *const () {
        Ok(Some(create_i8(-1)))
    } else if object_ptr as *const () > other_ptr as *const () {
        Ok(Some(create_i8(1)))
    } else {
        Ok(Some(create_i8(0)))
    }
}

fn obj_initalize(object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let mut object = object.borrow_mut();
    let mut arguments = Vec::new();
    for arg in 0..context.arg_count {
        arguments.push(context.arguments[arg].clone());
    }
    match context.vtable.take() {
        Some(vtable) => {
            object.initialize(arguments, vtable);
        },
        None => {
            object.initialize(arguments, VTable::new(HashMap::new()));
        }
    }
    Ok(None)
}



pub struct ObjectStruct {
    super_object: Option<ObjectBox>,
    fields: Box<[ObjectBox]>,
    vtable: VTable,
}

impl ObjectStruct {
    pub fn new(super_object: Option<ObjectBox>) -> ObjectBox {
        Rc::new(RefCell::new(ObjectStruct {
            super_object,
            fields: Box::new([]),
            vtable: VTable::new_empty(),
        })) as ObjectBox
    }
}


impl Object for ObjectStruct {
    fn get_vtable(&self) -> &VTable {
        &self.vtable
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        self.super_object.clone()
    }
    fn get_field(&self, index: usize) -> Option<ObjectBox> {
        self.fields.get(index).map(|field| field.clone())
    }
    fn set_field(&mut self, index: usize, value: ObjectBox) {
        self.fields[index] = value;
    }
    fn size(&self) -> Option<usize> {
        Some(self.fields.len())
    }
    fn duplicate(&self) -> ObjectBox {
        let mut fields = Vec::with_capacity(self.fields.len());
        for field in self.fields.iter() {
            fields.push(field.clone());
        }
        let object = ObjectStruct {
            super_object: self.super_object.clone(),
            fields: fields.into_boxed_slice(),
            vtable: self.vtable.clone(),
        };
        Rc::new(RefCell::new(object)) as ObjectBox
    }
    fn initialize(&mut self, arguments: Vec<ObjectBox>, vtable: VTable) {
        self.fields = arguments.into_boxed_slice();
        self.vtable.extend(vtable);
    }
}

#[derive(Clone)]
pub struct VTable {
    table: HashMap<String, Arc<Method>>,
}

impl VTable {
    pub fn new(table: HashMap<String, Arc<Method>>) -> VTable {
        VTable {
            table,
        }
    }
    pub fn new_empty() -> VTable {
        VTable {
            table: HashMap::new(),
        }
    }
    pub fn extend(&mut self, vtable: VTable) {
        self.table.extend(vtable.table);
    }
    pub fn get_method(&self, index: &str) -> Option<Arc<Method>> {
        self.table.get(index).cloned()
    }
    pub fn insert(&mut self, index: String, method: Arc<Method>) {
        self.table.insert(index, method);
    }
}



/*#[derive(Clone)]
pub struct Class {
    super_class: Option<Arc<Class>>,
}

impl Class {
    pub fn new(super_class: Option<Arc<Class>>) -> Class {
        Class {
            super_class,
        }
    }
    pub fn get_method(&self, index: &str) -> Option<Arc<Method>> {
        self.methods.get_method(index)
    }
    pub fn get_vtable(&self) -> VTable {
        self.methods.clone()
    }
}*/

unsafe impl Send for Method {}
unsafe impl Sync for Method {}

pub enum Method {
    RustMethod {
        fun: Box<dyn Fn(ObjectBox, &mut ContextData) -> Result<Option<ObjectBox>, Fault>>,
    },
    BytecodeMethod {
        block: ObjectBox,
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
    super_object: ObjectBox,
    vtable: VTable,
    index: String,
}


impl Message {
    /*pub fn make_class(parent: Arc<Class>) -> Class {
        let methods = HashMap::new();
        Class::new(Some(parent), methods)
    }*/
    pub fn make_object(parent: ObjectBox, 
                       index: String) -> ObjectBox {
        let message = Message {
            super_object: parent,
            index,
            vtable: VTable::new(HashMap::new()),
        };
        Rc::new(RefCell::new(message)) as ObjectBox
    }
}


impl Object for Message {
    fn get_vtable(&self) -> &VTable {
        &self.vtable
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        Some(self.super_object.clone())
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox> {
        panic!("Message does not have fields");
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox) {
        panic!("Message does not have fields");
    }
    fn size(&self) -> Option<usize> {
        panic!("Message does not have a size");
    }
    fn duplicate(&self) -> ObjectBox {
        let message = Message {
            super_object: self.super_object.clone(),
            index: self.index.clone(),
            vtable: self.vtable.clone(),
        };
        Rc::new(RefCell::new(message)) as ObjectBox
    }
    fn initialize(&mut self, _arguments: Vec<ObjectBox>, vtable: VTable) {
        self.vtable.extend(vtable);
    }
}

lazy_static! {
    static ref CLASSES: RwLock<ObjectFactory> = {
        let factory = ObjectFactory::new();
        RwLock::new(factory)
    };
}



pub struct ObjectFactory {
    //classes: HashMap<String, Arc<Class>>,
    parents: HashMap<String, String>,
}

impl ObjectFactory {
    fn new() -> ObjectFactory {
        let mut context = ObjectFactory {
            //classes: HashMap::new(),
            parents: HashMap::new(),
        };
        
        context.parents.insert(String::from("Message"), String::from("Object"));
        context.parents.insert(String::from("Number"), String::from("Object"));
        context.parents.insert(String::from("Integer"), String::from("Number"));
        context.parents.insert(String::from("Float"), String::from("Number"));
        context.parents.insert(String::from("String"), String::from("Object"));
        context.parents.insert(String::from("Char"), String::from("Object"));
        context.parents.insert(String::from("Stack"), String::from("Object"));
        context.parents.insert(String::from("Block"), String::from("Object"));
        context.parents.insert(String::from("Logger"), String::from("Object"));
        context.parents.insert(String::from("I64"), String::from("Integer"));
        context.parents.insert(String::from("U64"), String::from("Integer"));
        context.parents.insert(String::from("I32"), String::from("Integer"));
        context.parents.insert(String::from("U32"), String::from("Integer"));
        context.parents.insert(String::from("I16"), String::from("Integer"));
        context.parents.insert(String::from("U16"), String::from("Integer"));
        context.parents.insert(String::from("I8"), String::from("Integer"));
        context.parents.insert(String::from("U8"), String::from("Integer"));
        context.parents.insert(String::from("F64"), String::from("Float"));
        context.parents.insert(String::from("F32"), String::from("Float"));
        context.parents.insert(String::from("Boolean"), String::from("Object"));


        context
    }
    fn create_base_object(&self) -> ObjectBox {
        BaseObject::make_object()
    }
    fn create_boolean(&self, value: bool) -> ObjectBox {
        let boolean = BooleanObject::make_object(self.create_base_object(), value);
        let mut object = boolean.borrow_mut();
        object.initialize(vec![], VTable::new_empty());
        drop(object);
        boolean
    }
    fn create_number(&self) -> ObjectBox {
        let number = NumberObject::make_object(self.create_base_object());
        let mut object = number.borrow_mut();
        object.initialize(vec![], VTable::new_empty());
        drop(object);
        number
    }
    fn create_integer(&self) -> ObjectBox {
        let number = IntegerObject::make_object(self.create_number());
        let mut object = number.borrow_mut();
        object.initialize(vec![], VTable::new_empty());
        drop(object);
        number
    }
    fn create_i64(&self, value: i64) -> ObjectBox {
        let number = I64Object::make_object(self.create_integer(), value);
        let mut object = number.borrow_mut();
        object.initialize(vec![], VTable::new_empty());
        drop(object);
        number

    }
    fn create_u64(&self, value: u64) -> ObjectBox {
        let number = U64Object::make_object(self.create_integer(), value);
        let mut object = number.borrow_mut();
        object.initialize(vec![], VTable::new_empty());
        drop(object);
        number

    }
    fn create_i32(&self, value: i32) -> ObjectBox {
        let number = I32Object::make_object(self.create_integer(), value);
        let mut object = number.borrow_mut();
        object.initialize(vec![], VTable::new_empty());
        drop(object);
        number

    }
    fn create_u32(&self, value: u32) -> ObjectBox {
        let number = U32Object::make_object(self.create_integer(), value);
        let mut object = number.borrow_mut();
        object.initialize(vec![], VTable::new_empty());
        drop(object);
        number

    }
    fn create_i16(&self, value: i16) -> ObjectBox {
        let number = I16Object::make_object(self.create_integer(), value);
        let mut object = number.borrow_mut();
        object.initialize(vec![], VTable::new_empty());
        drop(object);
        number

    }
    fn create_u16(&self, value: u16) -> ObjectBox {
        let number = U16Object::make_object(self.create_integer(), value);
        let mut object = number.borrow_mut();
        object.initialize(vec![], VTable::new_empty());
        drop(object);
        number

    }
    fn create_i8(&self, value: i8) -> ObjectBox {
        let number = I8Object::make_object(self.create_integer(), value);
        let mut object = number.borrow_mut();
        object.initialize(vec![], VTable::new_empty());
        drop(object);
        number

    }
    fn create_u8(&self, value: u8) -> ObjectBox {
        let number = U8Object::make_object(self.create_integer(), value);
        let mut object = number.borrow_mut();
        object.initialize(vec![], VTable::new_empty());
        drop(object);
        number

    }
    fn create_float(&self) -> ObjectBox {
        let number = FloatObject::make_object(self.create_number());
        let mut object = number.borrow_mut();
        object.initialize(vec![], VTable::new_empty());
        drop(object);
        number

    }
    fn create_f64(&self, value: f64) -> ObjectBox {
        let number = F64Object::make_object(self.create_float(), value);
        let mut object = number.borrow_mut();
        object.initialize(vec![], VTable::new_empty());
        drop(object);
        number

    }
    fn create_f32(&self, value: f32) -> ObjectBox {
        let number = F32Object::make_object(self.create_float(), value);
        let mut object = number.borrow_mut();
        object.initialize(vec![], VTable::new_empty());
        drop(object);
        number

    }
    fn create_string(&self, value: String) -> ObjectBox {
        let string = StringObject::make_object(self.create_base_object(), value);
        let mut object = string.borrow_mut();
        object.initialize(vec![], VTable::new_empty());
        drop(object);
        string
    }
    fn create_character(&self, value: char) -> ObjectBox {
        let character = CharacterObject::make_object(self.create_base_object(), value);
        let mut object = character.borrow_mut();
        object.initialize(vec![], VTable::new_empty());
        drop(object);
        character
    }
    fn create_message(&self, index: &str) -> ObjectBox {
        let msg = Message::make_object(self.create_base_object(), index.to_string());
        let mut object = msg.borrow_mut();
        object.initialize(vec![], VTable::new_empty());
        drop(object);
        msg
    }
    fn create_logger(&self) -> ObjectBox {
        let log = Logger::make_object(self.create_base_object());
        let mut object = log .borrow_mut();
        object.initialize(vec![], VTable::new_empty());
        drop(object);
        log
    }
    fn init_stack(&self) -> ObjectBox {
        let framedata = vec![Context::make_object()];
        let frame = vec![stack::Stack::make_object_with_stack(self.create_base_object(),framedata)];
        let stack = stack::Stack::make_object_with_stack(self.create_base_object(), frame);
        let mut object = stack.borrow_mut();
        object.initialize(vec![], VTable::new_empty());
        drop(object);
        stack
    }
    fn create_stack(&self) -> ObjectBox {
        let stack = stack::Stack::make_object(self.create_base_object());
        let mut object = stack.borrow_mut();
        object.initialize(vec![], VTable::new_empty());
        drop(object);
        stack
    }

    fn make_parent(&self, name: &str) -> Result<ObjectBox, Fault> {
        self.create_object(self.parents.get(name).ok_or(Fault::InvalidType)?, &[])
    }
    
    fn create_object(&self, name: &str, arguments: &[ObjectBox]) -> Result<ObjectBox, Fault> {
        match name {
            "Object" => Ok(self.create_base_object()),
            "Number" => Ok(self.create_number()),
            "Integer" => Ok(self.create_integer()),
            "Float" => Ok(self.create_float()),
            "I64" => Ok(self.create_i64(0)),
            "U64" => Ok(self.create_u64(0)),
            "I32" => Ok(self.create_i32(0)),
            "U32" => Ok(self.create_u32(0)),
            "I16" => Ok(self.create_i16(0)),
            "U16" => Ok(self.create_u16(0)),
            "I8" => Ok(self.create_i8(0)),
            "U8" => Ok(self.create_u8(0)),
            "F64" => Ok(self.create_f64(0.0)),
            "F32" => Ok(self.create_f32(0.0)),
            "String" => Ok(self.create_string("".to_string())),
            "Char" => Ok(self.create_character(' ')),
            "Message" => {
                if arguments.len() == 1 {
                    let message = arguments[0].borrow();
                    let message = message.downcast_ref::<StringObject>().ok_or(Fault::InvalidType)?;
                    Ok(self.create_message(&message.value))
                } else {
                    Err(Fault::InvalidType)
                }
            },
            "Logger" => Ok(self.create_logger()),
            "Stack" => Ok(self.create_stack()),
            x => {
                let object = ObjectStruct::new(Some(self.make_parent(x)?));
                let mut object_mut = object.borrow_mut();
                drop(object_mut);
                Ok(object)
            }
        }
    }
}

fn get_factory<'a>() -> std::sync::RwLockReadGuard<'a, ObjectFactory> {
    loop {
        match CLASSES.try_read() {
            Ok(factory) => {
                return factory;
            }
            Err(_) => {
                std::thread::yield_now();
            }
        }
    }
}

fn get_factory_mut<'a>() -> std::sync::RwLockWriteGuard<'a, ObjectFactory> {
    loop {
        match CLASSES.try_write() {
            Ok(factory) => {
                return factory;
            }
            Err(_) => {
                std::thread::yield_now();
            }
        }
    }
}

pub fn create_base_object() -> ObjectBox {
    get_factory().create_base_object()
}

pub fn create_boolean(value: bool) -> ObjectBox {
    get_factory().create_boolean(value)
}

pub fn create_i64(value: i64) -> ObjectBox {
    get_factory().create_i64(value)
}

pub fn create_u64(value: u64) -> ObjectBox {
    get_factory().create_u64(value)
}

pub fn create_i32(value: i32) -> ObjectBox {
    get_factory().create_i32(value)
}

pub fn create_u32(value: u32) -> ObjectBox {
    get_factory().create_u32(value)
}

pub fn create_i16(value: i16) -> ObjectBox {
    get_factory().create_i16(value)
}

pub fn create_u16(value: u16) -> ObjectBox {
    get_factory().create_u16(value)
}

pub fn create_i8(value: i8) -> ObjectBox {
    get_factory().create_i8(value)
}

pub fn create_u8(value: u8) -> ObjectBox {
    get_factory().create_u8(value)
}

pub fn create_f64(value: f64) -> ObjectBox {
    get_factory().create_f64(value)
}

pub fn create_f32(value: f32) -> ObjectBox {
    get_factory().create_f32(value)
}

pub fn create_string(value: String) -> ObjectBox {
    get_factory().create_string(value)
}

pub fn create_character(value: char) -> ObjectBox {
    get_factory().create_character(value)
}

pub fn create_message(index: &str) -> ObjectBox {
    get_factory().create_message(index)
}

pub fn create_logger() -> ObjectBox {
    get_factory().create_logger()
}

pub fn init_stack() -> ObjectBox {
    get_factory().init_stack()
}

pub fn create_stack() -> ObjectBox {
    get_factory().create_stack()
}


pub fn create_object(name: &str, arguments: &[ObjectBox]) -> Result<Option<ObjectBox>, Fault> {
    let factory = get_factory();
    factory.create_object(name, arguments).map(|object| Some(object))
}


pub struct ContextData {
    pub stack: ObjectBox,
    pub arguments: Vec<ObjectBox>,
    pub receiver: Option<ObjectBox>,
    pub arg_count: usize,
    pub vtable: Option<VTable>,
}

impl ContextData {
    pub fn new(stack: ObjectBox) -> ContextData {
        ContextData {
            stack,
            arguments: vec![],
            receiver: None,
            arg_count: 0,
            vtable: None,
        }
    }

    pub fn attach_receiver(&mut self, receiver: ObjectBox) {
        self.receiver = Some(receiver);
    }
    pub fn detach_receiver(&mut self) {
        self.receiver = None;
    }

    pub fn push_frame(&mut self, frame: Option<ObjectBox>) {
        match frame {
            Some(frame) => {
                let mut stack = self.stack.borrow_mut();
                let stack = stack.downcast_mut::<stack::Stack>().unwrap();
                stack.push(frame);
            },
            None => {
                let mut stack = self.stack.borrow_mut();
                let stack = stack.downcast_mut::<stack::Stack>().unwrap();
                let frame = create_object("Stack", &[]).unwrap().unwrap();
                stack.push(frame);
            }
        }
    }
    pub fn pop_frame(&mut self) -> Option<ObjectBox> {
        let mut stack = self.stack.borrow_mut();
        let stack = stack.downcast_mut::<stack::Stack>().unwrap();
        stack.pop()
    }
    pub fn push(&mut self, value: ObjectBox) {
        let stack = self.stack.borrow();
        let stack = stack.downcast_ref::<stack::Stack>().unwrap();
        let mut stack = stack.data.last().unwrap().borrow_mut();
        let stack = stack.downcast_mut::<stack::Stack>().unwrap();
        stack.push(value);
    }
    pub fn pop(&mut self) -> Option<ObjectBox> {
        let stack = self.stack.borrow();
        let stack = stack.downcast_ref::<stack::Stack>().unwrap();
        let mut stack = stack.data.last().unwrap().borrow_mut();
        let stack = stack.downcast_mut::<stack::Stack>().unwrap();
        stack.pop()
    }
    pub fn top(&self) -> Option<ObjectBox> {
        let stack = self.stack.borrow();
        let stack = stack.downcast_ref::<stack::Stack>().unwrap();
        let stack = stack.data.last().unwrap().borrow();
        let stack = stack.downcast_ref::<stack::Stack>().unwrap();
        stack.data.last().map(|x| x.clone())
    }
    pub fn get_argument(&self, index: usize) -> Option<ObjectBox> {
        self.arguments.get(index).map(|x| x.clone())
    }
    pub fn set_argument(&mut self, index: usize, value: ObjectBox) {
        if index >= self.arguments.len() {
            self.arguments.resize(index + 1, Nil::new());
        }
        self.arguments[index] = value;
    }
    pub fn set_arguments(&mut self, arguments: Vec<ObjectBox>) {
        for (index, argument) in arguments.iter().enumerate() {
            self.set_argument(index, argument.clone());
        }
        self.arg_count = arguments.len();
    }
}


pub struct Context {
    super_object: Option<ObjectBox>,
    vtable: VTable,
}

impl Context {
    /*fn make_class() -> Class {
        let mut methods = HashMap::new();
        methods.insert("new".to_string(), Arc::new(Method::RustMethod { fun: Box::new(context_new) }));
        Class::new(Some(parent), methods)
    }*/
    pub fn make_vtable() -> VTable {
        let mut methods = HashMap::new();
        methods.insert("new".to_string(), Arc::new(Method::RustMethod { fun: Box::new(context_new) }));
        VTable::new(methods)
    }

    pub fn new() -> Context {
        let context = Context {
            super_object: Some(create_base_object()),
            vtable: VTable::new_empty()
        };
        context
    }
    pub fn make_object() -> ObjectBox {
        let mut context = Context::new();
        context.vtable.extend(Context::make_vtable());
        Rc::new(RefCell::new(context)) as ObjectBox
    }

}

impl Object for Context {
    fn get_vtable(&self) -> &VTable {
        &self.vtable
    }
    fn get_super_object(&self) -> Option<ObjectBox> {
        self.super_object.clone()
    }
    fn get_field(&self, _index: usize) -> Option<ObjectBox> {
        panic!("Context does not have fields");
    }
    fn set_field(&mut self, _index: usize, _value: ObjectBox) {
        panic!("Context does not have fields");
    }
    fn size(&self) -> Option<usize> {
        panic!("Context does not have a size");
    }
    fn duplicate(&self) -> ObjectBox {
        Context::make_object()
    }
    fn initialize(&mut self, _arguments: Vec<ObjectBox>, vtable: VTable) {
        self.vtable.extend(Context::make_vtable());
        self.vtable.extend(vtable);
    }
}


fn context_new(_: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let string = context.arguments[0].clone();
    let string = string.borrow();
    let string = string.downcast_ref::<StringObject>().ok_or(Fault::InvalidType)?;
    return create_object(&string.value, &context.arguments[1..]);
}



pub fn object_clone(object: ObjectBox) -> ObjectBox {
    let borrowed_object = object.borrow();
    if let Some(obj) = borrowed_object.downcast_ref::<PrimitiveObject<i64>>() {
        let new_obj = obj.clone();
        return Rc::new(RefCell::new(new_obj)) as ObjectBox
    } else if let Some(obj) = borrowed_object.downcast_ref::<PrimitiveObject<u64>>() {
        return Rc::new(RefCell::new(obj.clone())) as ObjectBox
    } else if let Some(obj) = borrowed_object.downcast_ref::<PrimitiveObject<i32>>() {
        return Rc::new(RefCell::new(obj.clone())) as ObjectBox
    } else if let Some(obj) = borrowed_object.downcast_ref::<PrimitiveObject<u32>>() {
        return Rc::new(RefCell::new(obj.clone())) as ObjectBox
    } else if let Some(obj) = borrowed_object.downcast_ref::<PrimitiveObject<i16>>() {
        return Rc::new(RefCell::new(obj.clone())) as ObjectBox
    }  else if let Some(obj) = borrowed_object.downcast_ref::<PrimitiveObject<u16>>() {
        return Rc::new(RefCell::new(obj.clone())) as ObjectBox
    } else if let Some(obj) = borrowed_object.downcast_ref::<PrimitiveObject<i8>>() {
        return Rc::new(RefCell::new(obj.clone())) as ObjectBox
    } else if let Some(obj) = borrowed_object.downcast_ref::<PrimitiveObject<u8>>() {
        return Rc::new(RefCell::new(obj.clone())) as ObjectBox
    } else if let Some(obj) = borrowed_object.downcast_ref::<PrimitiveObject<bool>>() {
        return Rc::new(RefCell::new(obj.clone())) as ObjectBox
    } else {
        return object.clone()
    }
}
