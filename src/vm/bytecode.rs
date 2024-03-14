//! # How to write a compiler to this bytecode;
//! ## Things to keep track of
//! - The position of the context object at the start of each stack frame.
//! - The position of temporary variables that are stored globally.
//! - The positions of the arguments on the runtime stack.
//! - How Blocks' (closures) captures are put after the arguments in the temporary variables.
//! - The importance of running the init message on an object before it is used.
use super::binary::ToBinary;










#[derive(Clone)]
pub enum ByteCode {
    /// Halt the current execution
    Halt,
    /// Do nothing
    NoOp,
    /// Access a field of the object at the top of the stack and push it to the stack
    AccessField(usize),
    /// Access a temporary variable and push it to the stack
    AccessTemp(usize),
    /// Push a literal to the stack
    PushLiteral(Literal),
    /// Store the top of the stack in a field of the object
    StoreField(usize),
    /// Store the top of the stack in a temporary variable
    StoreTemp(usize),
    /// Send a message to an object
    /// The usize is the number of arguments to send
    /// The string is the name of the message to send
    SendMsg(usize, String),
    /// Send a message to the super object
    /// The usize is the number of arguments to send
    /// The string is the name of the message to send
    SendSuperMsg(usize, String),
    /// Perform a special instruction
    SpecialInstruction(SpecialInstruction),
    /// Get from the current runtime stack
    /// The first usize is the index of the stack frame
    /// The second usize is the index of the value in the stack frame
    /// Both are 0-indexed from the top of the stack
    GetStack(usize, usize)
}


impl ToBinary for ByteCode {
    fn to_binary(&self, string_table: Option<&mut super::binary::StringTable>) -> Vec<u8> {
        let string_table = string_table.expect("ByteCode::to_binary called without a StringTable");
        match self {
            ByteCode::Halt => vec![0],
            ByteCode::NoOp => vec![1],
            ByteCode::AccessField(idx) => {
                let mut binary = vec![2];
                binary.extend(idx.to_binary(None));
                binary
            },
            ByteCode::AccessTemp(idx) => {
                let mut binary = vec![3];
                binary.extend(idx.to_binary(None));
                binary
            },
            ByteCode::PushLiteral(lit) => {
                let mut binary = vec![4];
                binary.extend(lit.to_binary(Some(string_table)));
                binary
            },
            ByteCode::StoreField(idx) => {
                let mut binary = vec![5];
                binary.extend(idx.to_binary(None));
                binary
            },
            ByteCode::StoreTemp(idx) => {
                let mut binary = vec![6];
                binary.extend(idx.to_binary(None));
                binary
            },
            ByteCode::SendMsg(num_args, name) => {
                let idx = string_table.add_string(name.clone());
                let mut binary = vec![7];
                binary.extend(num_args.to_binary(None));
                binary.extend(idx.to_binary(None));
                binary
            },
            ByteCode::SendSuperMsg(num_args, name) => {
                let idx = string_table.add_string(name.clone());
                let mut binary = vec![8];
                binary.extend(num_args.to_binary(None));
                binary.extend(idx.to_binary(None));
                binary
            },
            ByteCode::SpecialInstruction(inst) => {
                let mut binary = vec![9];
                binary.extend(inst.to_binary(None));
                binary
            },
            ByteCode::GetStack(frame, idx) => {
                let mut binary = vec![10];
                binary.extend(frame.to_binary(None));
                binary.extend(idx.to_binary(None));
                binary
            }
        }
    }
}

#[derive(Clone)]
pub enum SpecialInstruction {
    /// Duplicate the top of the stack
    DupStack,
    /// Discard the top of the stack
    DiscardStack,
    /// Return from a method and push the top of the stack to the
    /// previous stack frame
    ReturnStack,
    /// Return from a method
    Return,
    /// Pop the top of the stack, if true skip the next n instructions
    PopTrueSkip(usize),
    /// Pop the top of the stack, if false skip the next n instructions
    PopFalseSkip(usize),
    /// Pop the top of the stack, if true go back n instructions
    PopTrueBackSkip(usize),
    /// Pop the top of the stack, if false go back n instructions
    PopFalseBackSkip(usize),
    /// Skip the next n instructions
    Skip(usize),
    /// Go back n instructions
    BackSkip(usize),
}

impl ToBinary for SpecialInstruction {
    fn to_binary(&self, _string_table: Option<&mut super::binary::StringTable>) -> Vec<u8> {
        let mut output = Vec::new();
        match self {
            SpecialInstruction::DupStack => {
                output.push(0);
            },
            SpecialInstruction::DiscardStack => {
                output.push(1);
            },
            SpecialInstruction::ReturnStack => {
                output.push(2);
            },
            SpecialInstruction::Return => {
                output.push(3);
            },
            SpecialInstruction::PopTrueSkip(idx) => {
                output.push(4);
                output.extend(idx.to_binary(None));
            },
            SpecialInstruction::PopFalseSkip(idx) => {
                output.push(5);
                output.extend(idx.to_binary(None));
            },
            SpecialInstruction::PopTrueBackSkip(idx) => {
                output.push(6);
                output.extend(idx.to_binary(None));
            },
            SpecialInstruction::PopFalseBackSkip(idx) => {
                output.push(7);
                output.extend(idx.to_binary(None));
            },
            SpecialInstruction::Skip(idx) => {
                output.push(8);
                output.extend(idx.to_binary(None));
            },
            SpecialInstruction::BackSkip(idx) => {
                output.push(9);
                output.extend(idx.to_binary(None));
            },
        }
        output
    }
}

#[derive(Clone)]
pub enum Literal {
    String(String),
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
    Boolean(bool),
    Nil,
    ByteCode(Vec<ByteCode>),
}

impl ToBinary for Literal {
    fn to_binary(&self, string_table: Option<&mut super::binary::StringTable>) -> Vec<u8> {
        let mut output = Vec::new();
        match self {
            Literal::String(s) => {
                output.push(0);
                let string_table = string_table.expect("Literal::to_binary called without a StringTable");
                let idx = string_table.add_string(s.clone());
                output.extend_from_slice(&idx.to_binary(None));
            },
            Literal::I8(i) => {
                output.push(1);
                output.extend(i.to_le_bytes());
            },
            Literal::U8(u) => {
                output.push(2);
                output.extend(u.to_le_bytes());
            },
            Literal::I16(i) => {
                output.push(3);
                output.extend(i.to_le_bytes());
            },
            Literal::U16(u) => {
                output.push(4);
                output.extend(u.to_le_bytes());
            },
            Literal::I32(i) => {
                output.push(5);
                output.extend(i.to_le_bytes());
            },
            Literal::U32(u) => {
                output.push(6);
                output.extend(u.to_le_bytes());
            },
            Literal::I64(i) => {
                output.push(7);
                output.extend(i.to_le_bytes());
            },
            Literal::U64(u) => {
                output.push(8);
                output.extend(u.to_le_bytes());
            },
            Literal::F32(f) => {
                output.push(9);
                output.extend(f.to_le_bytes());
            },
            Literal::F64(f) => {
                output.push(10);
                output.extend(f.to_le_bytes());
            },
            Literal::Boolean(b) => {
                output.push(11);
                output.push(if *b { 1 } else { 0 });
            },
            Literal::Nil => {
                output.push(12);
            },
            Literal::ByteCode(bc) => {
                output.push(13);
                output.extend(bc.len().to_binary(None));
                let string_table = string_table.expect("Literal::to_binary called without a StringTable");
                for code in bc {
                    output.extend(code.to_binary(Some(string_table)));
                }
            },
                    
        }
        output
    }
}
