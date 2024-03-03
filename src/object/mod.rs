pub mod primitive;
pub mod stack;
pub mod bytecode;
pub mod interpreter;

use std::sync::Arc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub enum Fault {
    NotImplemented,
    InvalidOperation,
    InvalidType,
    DivideByZero,
}


pub type ObjectBox<T> = Rc<RefCell<T>>;



pub trait Object: downcast_rs::Downcast {
    fn get_class(&self) -> &Class;
    fn get_super_object(&self) -> Option<ObjectBox<dyn Object>>;
    fn get_field(&self, index: usize) -> Option<ObjectBox<dyn Object>>;
    fn set_field(&mut self, index: usize, value: ObjectBox<dyn Object>);
    fn size(&self) -> Option<usize>;
    fn handle_message(&self, message: &Message) -> Option<Arc<Method>> {
        let mut method = self.get_class().get_method(message.index);
        let class = self.get_class();
        let mut super_class = &class.super_class;
        while method.is_none() {
            if let Some(class) = super_class {
                method = class.get_method(message.index);
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
}
downcast_rs::impl_downcast!(Object);


pub struct Nil;

impl Nil {
    pub fn new() -> ObjectBox<dyn Object> {
        Rc::new(RefCell::new(Nil)) as ObjectBox<dyn Object>
    }
}

impl Object for Nil {
    fn get_class<'a>(&'a self) -> &Class {
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
}

pub struct BaseObject {}

impl BaseObject {
    pub fn make_class() -> Class {
        let methods = Vec::new();
        Class::new(None, methods)
    }

    pub fn make_object(class: Class) -> ObjectBox<dyn Object> {
        let object = ObjectStruct::new(class, Some(Nil::new()), 0);
        object
    }
}

pub struct ObjectStruct {
    class: Class,
    super_object: Option<ObjectBox<dyn Object>>,
    fields: Box<[ObjectBox<dyn Object>]>,
}

impl ObjectStruct {
    pub fn new(class: Class, super_object: Option<ObjectBox<dyn Object>>, size: usize) -> ObjectBox<dyn Object> {
        let fields = Vec::with_capacity(size);
        ObjectBox::new(RefCell::new(ObjectStruct {
            class,
            super_object,
            fields: fields.into_boxed_slice(),
        })) as ObjectBox<dyn Object>
    }
}


impl Object for ObjectStruct {
    fn get_class(&self) -> &Class {
        &self.class
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
}

#[derive(Clone)]
pub struct Class {
    super_class: Option<Box<Class>>,
    methods: Vec<Arc<Method>>,
}

impl Class {
    pub fn new(super_class: Option<Box<Class>>, methods: Vec<Arc<Method>>) -> Class {
        Class {
            super_class,
            methods,
        }
    }
    pub fn get_method(&self, index: usize) -> Option<Arc<Method>> {
        self.methods.get(index).cloned()
    }
    pub fn override_method(&mut self, index: usize, method: Arc<Method>) {
        //eprintln!("Overriding method");
        //eprintln!("{:?}", self.methods);
        self.methods[index] = method;
    }
}

pub enum Method {
    RustMethod {
        fun: Box<dyn Fn(ObjectBox<dyn Object>, &mut Context) -> Result<Option<ObjectBox<dyn Object>>, Fault>>,
    },
    BytecodeMethod {
    },
}

impl std::fmt::Debug for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Method::RustMethod { fun: _ } => write!(f, "RustMethod"),
            Method::BytecodeMethod {} => write!(f, "BytecodeMethod"),
        }
    }
}



pub struct Message {
    class: Class,
    super_object: ObjectBox<dyn Object>,
    index: usize,
}


impl Message {
    pub fn make_class(parent: Box<Class>) -> Class {
        let methods = vec![];
        Class::new(Some(parent), methods)
    }
    pub fn make_object(class: Class, 
                           parent: ObjectBox<dyn Object>, 
                           index: usize) -> ObjectBox<dyn Object> {
        let message = Message {
            class,
            super_object: parent,
            index,
        };
        ObjectBox::new(RefCell::new(message)) as ObjectBox<dyn Object>
    }
}


impl Object for Message {
    fn get_class(&self) -> &Class { 
        &self.class
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
}


pub struct Context {
    classes: HashMap<String, Box<Class>>,
    pub arguments: Vec<ObjectBox<dyn Object>>,
}

impl Context {
    /*fn make_class(parent: &'a Class) -> Class<'a> {
        let methods = vec![];
        Class::new("Context", Some(parent), methods)
    }*/

    pub fn new() -> Context {
        let base_object_class = Box::new(BaseObject::make_class());
        let mut classes = HashMap::new();
        classes.insert("Object".to_string(), base_object_class);
        let arguments = vec![];
        let context = Context {
            classes,
            arguments,
        };

        context
    }

    pub fn get_class(&self, name: &str) -> Option<&Box<Class>> {
        self.classes.get(name)
    }

    pub fn add_class(&mut self, name: &str, class: Class) {
        self.classes.insert(name.to_string(), Box::new(class));
    }

    pub fn create_base_object(&self) -> ObjectBox<dyn Object> {
        BaseObject::make_object(*self.get_class("Object").unwrap().clone())
    }
}
