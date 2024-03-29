pub mod primitive;
pub mod stack;
pub mod block;
pub mod string;
pub mod log;
pub mod vector;
pub mod system;

use lazy_static::lazy_static;
use std::sync::{Arc, Mutex, MutexGuard};
use std::collections::HashMap;
use std::sync::RwLock;

use crate::vm::bytecode::ByteCode;

use self::log::Logger;
use self::stack::Stack;
use self::primitive::boolean::BooleanObject;
use self::primitive::character::CharacterObject;
use self::primitive::float::{F32Object, F64Object, FloatObject};
use self::primitive::integer::{I16Object, I32Object, I64Object, I8Object, IntegerObject, U16Object, U32Object, U64Object, U8Object};
use self::primitive::{NumberObject, PrimitiveObject};
use self::string::StringObject;

#[derive(Debug)]
pub enum Fault {
    NotImplemented(String),
    InvalidOperation(String),
    InvalidType(String),
    DivideByZero,
    IO(std::io::Error),
    MethodNotFound(String),
}

impl std::fmt::Display for Fault {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Fault::NotImplemented(string) => write!(f, "Not implemented: {}", string),
            Fault::InvalidOperation(string) => write!(f, "Invalid operation: {}", string),
            Fault::InvalidType(string) => write!(f, "Invalid type: {}", string),
            Fault::DivideByZero => write!(f, "Divide by zero"),
            Fault::IO(e) => write!(f, "IO error: {}", e),
            Fault::MethodNotFound(name) => write!(f, "Method not found: {}", name),
        }
    }
}

impl std::error::Error for Fault {}

//pub type ObjectBox = Rc<RefCell<dyn Object>>;

unsafe impl Send for ObjectBox {}

#[derive(Clone)]
pub struct ObjectBox {
    pub data: Arc<Mutex<dyn Object>>,
}

impl ObjectBox {
    pub fn new<O: Object>(data: O) -> ObjectBox {
        ObjectBox {
            data: Arc::new(Mutex::new(data))
        }
    }

    pub fn borrow(&self) -> MutexGuard<dyn Object> {
        self.data.lock().expect("ObjectBox::borrow: lock poisoned")
    }

    pub fn borrow_mut(&self) -> MutexGuard<dyn Object> {
        self.data.lock().expect("ObjectBox::borrow_mut: lock poisoned")
    }

    pub fn as_ptr(&self) -> *const () {
        Arc::as_ptr(&self.data) as *const ()
    }
}


/// This object defines the interface for all objects in the system.
/// This is so that all objects are trait objects.
pub trait Object: downcast_rs::Downcast {
    /// Get the vtable for the object
    fn get_vtable(&self) -> &VTable;
    /// Get the super object
    fn get_super_object(&self) -> Option<ObjectBox>;
    /// Get a field from the object
    fn get_field(&self, index: usize) -> Option<ObjectBox>;
    /// Set a field in the object
    fn set_field(&mut self, index: usize, value: ObjectBox);
    /// Get the size of the object. This might get removed in the future since it's not used.
    fn size(&self) -> Option<usize>;
    /// Handle a message
    /// This method gets a method from a vtable and if it doesn't find it, it looks in the super object.
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
    /// This method processes a message and returns a method to call.
    /// This is the method that gets called when a message is sent to an object.
    /// The message is an object so it can be anything but we can assume it's a message.
    fn process_message(&self, message: ObjectBox) -> Option<Arc<Method>> {
        let message = message.borrow();
        if let Some(message) = (&*message).downcast_ref::<Message>() {
            self.handle_message(message)
        } else {
            panic!("Object::process_message: message is not a Message")
        }
    }
    /// Duplicate the object
    /// This method duplicates the object. This is used for cloning objects.
    fn duplicate(&self) -> ObjectBox;
    /// Initialize the object
    /// This method initializes the object. This should get called when the init message is passed
    /// into the object.
    fn initialize(&mut self, arguments: Vec<ObjectBox>, vtable: VTable);
    
}
downcast_rs::impl_downcast!(Object);

/// Nil
/// the Nil object is a special object that represents nothing. It's used as the super object for
/// the base object.
pub struct Nil;

impl Nil {
    pub fn new() -> ObjectBox {
        ObjectBox::new(Nil)
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
        ()
    }
}

/// BaseObject
/// The BaseObject is the base object for all objects. It's the object that all objects inherit from.
/// It contains the methods clone, equals, to_string, order, and init,
pub struct BaseObject {
    super_object: Option<ObjectBox>,
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
        methods.insert("to_string".to_string(), Arc::new(Method::RustMethod { fun: Box::new(obj_to_string) }));
        methods.insert("order".to_string(), Arc::new(Method::RustMethod { fun: Box::new(obj_order) }));
        methods.insert("init".to_string(), Arc::new(Method::RustMethod { fun: Box::new(obj_initalize) }));
        VTable::new(methods)
    }

    pub fn make_object(parent: ObjectBox) -> ObjectBox {
        ObjectBox::new(BaseObject {vtable: VTable::new_empty(), super_object: Some(parent)})
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
        let obje = BaseObject::make_object(self.super_object.as_ref().unwrap().borrow().duplicate());
        let mut obj = obje.borrow_mut();
        obj.initialize(vec![], self.vtable.clone());
        drop(obj);
        obje
    }
    fn initialize(&mut self, _arguments: Vec<ObjectBox>, vtable: VTable) {
        self.vtable.extend(BaseObject::make_vtable());
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

/*fn obj_hash(object: ObjectBox, _: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let object = object.as_ptr();
    let string = format!("{:p}", object);
    let mut hasher = std::hash::DefaultHasher::new();
    string.hash(&mut hasher);
    let hash = hasher.finish();
    Ok(Some(create_u64(hash as u64)))
}*/

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


/// ObjectStruct
/// This is the object that gets created when a class is created. It contains the vtable, fields, and
/// super object.
pub struct ObjectStruct {
    class: Option<Arc<Class>>,
    super_object: Option<ObjectBox>,
    fields: Box<[ObjectBox]>,
    vtable: VTable,
}

impl ObjectStruct {
    pub fn new(class: Option<Arc<Class>>, super_object: Option<ObjectBox>) -> ObjectBox {
        ObjectBox::new(ObjectStruct {
            class,
            super_object,
            fields: Box::new([]),
            vtable: VTable::new_empty(),
        })
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
            class: self.class.clone(),
            super_object: self.super_object.clone(),
            fields: fields.into_boxed_slice(),
            vtable: self.vtable.clone(),
        };
        ObjectBox::new(object)
    }
    fn initialize(&mut self, arguments: Vec<ObjectBox>, vtable: VTable) {
        self.fields = arguments.into_boxed_slice();
        self.vtable.extend(self.class.as_ref().unwrap().get_vtable());
        self.vtable.extend(vtable);
        let mut super_object = self.super_object.clone();
        let mut overrides = self.class.as_ref().unwrap().get_overrides();
        while super_object.is_some() {
            match super_object {
                Some(super_object_ref) => {
                    let mut super_object_ref2 = super_object_ref.borrow_mut();
                    let vtable = match overrides.pop() {
                        Some(vtable) => vtable,
                        None => VTable::new_empty(),
                    };
                    super_object_ref2.initialize(vec![], vtable);
                    drop(super_object_ref2);
                    super_object = super_object_ref.borrow().get_super_object();
                }
                None => {}
            }
        }
    }
}

/// VTable
/// This is the vtable for an object. It contains a hashmap of methods.
/// This is so that we can call methods on an object.
#[derive(Clone, Debug)]
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
    pub fn empty(&self) -> bool {
        self.table.is_empty()
    }
}

impl crate::vm::binary::ToBinary for VTable {
    fn to_binary(&self, string_table: Option<&mut crate::vm::binary::StringTable>) -> Vec<u8> {
        let mut output = Vec::new(); 
        let string_table = string_table.expect("VTable::to_binary called without a StringTable");
        for (name, method) in self.table.iter() {
            let idx = string_table.add_string(name.clone());
            output.extend_from_slice(idx.to_binary(None).as_slice());
            method.to_binary(None);
        }
        output
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

/// Class
/// A Class is a prototype for objects. It contains a vtable of methods and a parent class name.
/// This is so that we can build the object.
/// This is to be used for programmer created classes where we know nothing about them.
pub struct Class {
    /// The parent class name
    parent: Option<String>,
    /// The vtable of methods
    methods: VTable,
    /// The vtables of the parent classes
    /// This is sorted by depth with the deepest at the start and the shallowest at the end.
    overrides: Vec<VTable>,
}

impl Class {
    pub fn new(parent: Option<&str>, methods: VTable, overrides: Vec<VTable>) -> Class {
        Class {
            parent: parent.map(|x| x.to_string()),
            methods,
            overrides,
        }
    }
    pub fn get_method(&self, index: &str) -> Option<Arc<Method>> {
        self.methods.get_method(index)
    }
    pub fn get_vtable(&self) -> VTable {
        self.methods.clone()
    }
    pub fn get_overrides(&self) -> Vec<VTable> {
        self.overrides.clone()
    }
}

impl crate::vm::binary::ToBinary for Class {
    fn to_binary(&self, string_table: Option<&mut crate::vm::binary::StringTable>) -> Vec<u8> {
        let mut output = Vec::new(); 
        let string_table = string_table.expect("Class::to_binary called without a StringTable");
        if let Some(parent) = &self.parent {
            output.extend_from_slice(&[0x01]);
            output.extend_from_slice(parent.as_bytes());
        } else {
            output.extend_from_slice(&[0x00]);
        }
        self.methods.to_binary(Some(string_table));

        output.extend_from_slice(self.overrides.len().to_binary(None).as_slice());
        for override_ in self.overrides.iter().rev() {
            override_.to_binary(Some(string_table));
        }

        output
    }
}


unsafe impl Send for Method {}
unsafe impl Sync for Method {}
/// Method
/// A method is a function that an object can respond to. It can be a Rust function or a Block
/// object.
pub enum Method {
    RustMethod {
        fun: Box<dyn Fn(ObjectBox, &mut ContextData) -> Result<Option<ObjectBox>, Fault>>,
    },
    BytecodeMethod {
        block: ObjectBox,
    },
}

impl Method {
    pub fn call(&self, object: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
        match self {
            Method::RustMethod { fun } => {
                fun(object, context)
            },
            Method::BytecodeMethod { block } => {
                let block = block.borrow();
                let block = block.downcast_ref::<block::Block>().unwrap();
                block.call(context)
            }
        }
    }
}

impl crate::vm::binary::ToBinary for Method {
    fn to_binary(&self, _: Option<&mut crate::vm::binary::StringTable>) -> Vec<u8> {
        let mut output = Vec::new(); 
        match self {
            Method::RustMethod { fun: _ } => {
                panic!("Cannot convert a RustMethod to binary");
            },
            Method::BytecodeMethod { block } => {
                let block = block.borrow();
                let bytecode = block.downcast_ref::<block::Block>().unwrap().bytecode.clone();
                output.extend_from_slice(bytecode.len().to_binary(None).as_slice());
                for byte in bytecode.iter() {
                    output.extend_from_slice(byte.to_binary(None).as_slice());
                }
            }
        }
        output
    }
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
        ObjectBox::new(message)
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
        ObjectBox::new(message)
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
    classes: HashMap<String, Arc<Class>>,
    parents: HashMap<String, String>,
}

impl ObjectFactory {
    fn new() -> ObjectFactory {
        let mut context = ObjectFactory {
            classes: HashMap::new(),
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
        context.parents.insert(String::from("Vector"), String::from("Object"));
        context.parents.insert(String::from("System"), String::from("Object"));


        context
    }
    fn add_class(&mut self, name: &str, class: Class) {
        match &class.parent {
            Some(parent) => {
                self.parents.insert(name.to_string(), parent.to_string());
            }
            None => {}
        }
        let class = Arc::new(class);
        self.classes.insert(name.to_string(), class);
    }
    fn get_class(&self, name: &str) -> Option<Arc<Class>> {
        self.classes.get(name).cloned()
    } 

    fn create_base_object(&self) -> ObjectBox {
        let object = BaseObject::make_object(Nil::new());
        let mut object_mut = object.borrow_mut();
        object_mut.initialize(vec![], VTable::new_empty());
        drop(object_mut);
        object
    }
    fn create_boolean(&self, value: bool) -> ObjectBox {
        BooleanObject::make_object(self.create_base_object(), value)
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
        I64Object::make_object(self.create_integer(), value) 
    }
    fn create_u64(&self, value: u64) -> ObjectBox {
        U64Object::make_object(self.create_integer(), value)
    }
    fn create_i32(&self, value: i32) -> ObjectBox {
        I32Object::make_object(self.create_integer(), value)
    }
    fn create_u32(&self, value: u32) -> ObjectBox {
        U32Object::make_object(self.create_integer(), value)
    }
    fn create_i16(&self, value: i16) -> ObjectBox {
        I16Object::make_object(self.create_integer(), value) 
    }
    fn create_u16(&self, value: u16) -> ObjectBox {
        U16Object::make_object(self.create_integer(), value)
    }
    fn create_i8(&self, value: i8) -> ObjectBox {
        I8Object::make_object(self.create_integer(), value)
    }
    fn create_u8(&self, value: u8) -> ObjectBox {
        U8Object::make_object(self.create_integer(), value)
    }
    fn create_float(&self) -> ObjectBox {
        FloatObject::make_object(self.create_number())
    }
    fn create_f64(&self, value: f64) -> ObjectBox {
        F64Object::make_object(self.create_float(), value) 
    }
    fn create_f32(&self, value: f32) -> ObjectBox {
        F32Object::make_object(self.create_float(), value)
    }
    fn create_string(&self, value: String) -> ObjectBox {
        StringObject::make_object(self.create_base_object(), value)    
    }
    fn create_character(&self, value: char) -> ObjectBox {
        CharacterObject::make_object(self.create_base_object(), value)    
    }
    fn create_message(&self, index: &str) -> ObjectBox {
        Message::make_object(self.create_base_object(), index.to_string()) 
    }
    fn create_logger(&self) -> ObjectBox {
        Logger::make_object(self.create_base_object())
    }
    fn init_stack(&self) -> ObjectBox {
        let context = Context::make_object();
        let mut context_mut = context.borrow_mut();
        context_mut.initialize(vec![], VTable::new_empty());
        drop(context_mut);
        let framedata = vec![context];
        let frame = vec![stack::Stack::make_object_with_stack(self.create_base_object(),framedata)];
        stack::Stack::make_object_with_stack(self.create_base_object(), frame) 
    }
    fn create_stack(&self) -> ObjectBox {
        stack::Stack::make_object(self.create_base_object())
    }
    fn create_block(&self, bytecode: Vec<ByteCode>) -> ObjectBox {
        block::Block::make_object(self.create_base_object(), bytecode)
    }
    fn create_vector(&self, vector: Vec<ObjectBox>) -> ObjectBox {
        vector::VectorObject::make_object(self.create_base_object(), vector.into())
    }
    fn create_system(&self) -> ObjectBox {
        system::System::make_object(self.create_base_object())
    }

    fn make_parent(&self, name: &str) -> Result<ObjectBox, Fault> {
        self.create_object(self.parents.get(name).ok_or(Fault::InvalidType(format!("object not found: {}", name)))?, &[])
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
            "String" => Ok(self.create_string("".to_string())),//TODO: add way to create it from vector
            "Char" => Ok(self.create_character(' ')),
            "Message" => {
                if arguments.len() == 1 {
                    let message = arguments[0].borrow();
                    let message = message.downcast_ref::<StringObject>().ok_or(Fault::InvalidType(format!("argument wasn't a string")))?;
                    Ok(self.create_message(&message.value))
                } else {
                    Err(Fault::InvalidType(format!("expected 1 argument, got {}", arguments.len())))
                }
            },
            "Logger" => Ok(self.create_logger()),
            "Stack" => Ok(self.create_stack()),
            "Block" => Ok(self.create_block(vec![])),
            "Vector" => {
                let vector = Vec::new();
                Ok(self.create_vector(vector))
            },
            "System" => Ok(self.create_system()),
            x => {
                let object = ObjectStruct::new(self.get_class(x), Some(self.make_parent(x)?));
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

pub fn add_class(name: &str, class: Class) {
    let mut factory = get_factory_mut();
    factory.add_class(name, class);
}

pub fn create_base_object() -> ObjectBox {
    get_factory().create_base_object()
}

pub fn create_boolean(value: bool) -> ObjectBox {
    let boolean = get_factory().create_boolean(value);
    let mut object = boolean.borrow_mut();
    object.initialize(vec![], VTable::new_empty());
    drop(object);
    boolean
}

pub fn create_i64(value: i64) -> ObjectBox {
    let integer = get_factory().create_i64(value);
    let mut object = integer.borrow_mut();
    object.initialize(vec![], VTable::new_empty());
    drop(object);
    integer
}

pub fn create_u64(value: u64) -> ObjectBox {
    let integer = get_factory().create_u64(value);
    let mut object = integer.borrow_mut();
    object.initialize(vec![], VTable::new_empty());
    drop(object);
    integer
}

pub fn create_i32(value: i32) -> ObjectBox {
    let integer = get_factory().create_i32(value);
    let mut object = integer.borrow_mut();
    object.initialize(vec![], VTable::new_empty());
    drop(object);
    integer
}

pub fn create_u32(value: u32) -> ObjectBox {
    let integer = get_factory().create_u32(value);
    let mut object = integer.borrow_mut();
    object.initialize(vec![], VTable::new_empty());
    drop(object);
    integer
}

pub fn create_i16(value: i16) -> ObjectBox {
    let integer = get_factory().create_i16(value);
    let mut object = integer.borrow_mut();
    object.initialize(vec![], VTable::new_empty());
    drop(object);
    integer
}

pub fn create_u16(value: u16) -> ObjectBox {
    let integer = get_factory().create_u16(value);
    let mut object = integer.borrow_mut();
    object.initialize(vec![], VTable::new_empty());
    drop(object);
    integer
}

pub fn create_i8(value: i8) -> ObjectBox {
    let integer = get_factory().create_i8(value);
    let mut object = integer.borrow_mut();
    object.initialize(vec![], VTable::new_empty());
    drop(object);
    integer
}

pub fn create_u8(value: u8) -> ObjectBox {
    let integer = get_factory().create_u8(value);
    let mut object = integer.borrow_mut();
    object.initialize(vec![], VTable::new_empty());
    drop(object);
    integer
}

pub fn create_f64(value: f64) -> ObjectBox {
    let float = get_factory().create_f64(value);
    let mut object = float.borrow_mut();
    object.initialize(vec![], VTable::new_empty());
    drop(object);
    float
}

pub fn create_f32(value: f32) -> ObjectBox {
    let float = get_factory().create_f32(value);
    let mut object = float.borrow_mut();
    object.initialize(vec![], VTable::new_empty());
    drop(object);
    float
}

pub fn create_string(value: String) -> ObjectBox {
    let string = get_factory().create_string(value);
    let mut object = string.borrow_mut();
    object.initialize(vec![], VTable::new_empty());
    drop(object);
    string
}

pub fn create_character(value: char) -> ObjectBox {
    let character = get_factory().create_character(value);
    let mut object = character.borrow_mut();
    object.initialize(vec![], VTable::new_empty());
    drop(object);
    character
}

pub fn create_message(index: &str) -> ObjectBox {
    let msg = get_factory().create_message(index);
    let mut object = msg.borrow_mut();
    object.initialize(vec![], VTable::new_empty());
    drop(object);
    msg
}

pub fn create_logger() -> ObjectBox {
    get_factory().create_logger()
}

pub fn init_stack() -> ObjectBox {
    let stack = get_factory().init_stack();
    let mut object = stack.borrow_mut();
    object.initialize(vec![], VTable::new_empty());
    drop(object);
    stack
}

pub fn create_stack() -> ObjectBox {
    let stack = get_factory().create_stack();
    let mut object = stack.borrow_mut();
    object.initialize(vec![], VTable::new_empty());
    drop(object);
    stack
}

pub fn create_block(bytecode: Vec<ByteCode>) -> ObjectBox {
    let block = get_factory().create_block(bytecode);
    let mut object = block.borrow_mut();
    object.initialize(vec![], VTable::new_empty());
    drop(object);
    block
}

pub fn create_vector(vector: Vec<ObjectBox>) -> ObjectBox {
    let vector = get_factory().create_vector(vector);
    let mut object = vector.borrow_mut();
    object.initialize(vec![], VTable::new_empty());
    drop(object);
    vector
}

pub fn create_system() -> ObjectBox {
    let system = get_factory().create_system();
    let mut object = system.borrow_mut();
    object.initialize(vec![], VTable::new_empty());
    drop(object);
    system
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
    pub code: Option<Arc<Vec<ByteCode>>>
}

impl ContextData {
    pub fn new(stack: ObjectBox) -> ContextData {
        ContextData {
            stack,
            arguments: vec![],
            receiver: None,
            arg_count: 0,
            vtable: None,
            code: None,
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
    pub fn attach_code(&mut self, code: Arc<Vec<ByteCode>>) {
        self.code = Some(code);
    }
    pub fn detach_code(&mut self) -> Option<Arc<Vec<ByteCode>>> {
        self.code.take()
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
        methods.insert("stack".to_string(), Arc::new(Method::RustMethod { fun: Box::new(context_get_stack) }));
        methods.insert("current_frame".to_string(), Arc::new(Method::RustMethod { fun: Box::new(context_get_current_frame) }));
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
        ObjectBox::new(context)
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
    let string = string.downcast_ref::<StringObject>().ok_or(Fault::InvalidType(format!("argument was not a string")))?;
    return create_object(&string.value, &context.arguments[1..]);
}

fn context_get_stack(_: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let stack = context.stack.clone();
    Ok(Some(stack))
}

fn context_get_current_frame(_: ObjectBox, context: &mut ContextData) -> Result<Option<ObjectBox>, Fault> {
    let stack = context.stack.clone();
    let stack = stack.borrow();
    let stack = stack.downcast_ref::<Stack>();
    let stack = stack.expect("Expected stack");
    let frame = stack.index(0).unwrap();
    Ok(Some(frame))
}


/*pub fn object_clone(object: ObjectBox) -> ObjectBox {
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
}*/
