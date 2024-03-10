use super::binary::ToBinary;










#[derive(Clone)]
pub enum ByteCode {
    Halt,
    NoOp,
    AccessField(usize),
    AccessTemp(usize),
    PushLiteral(Literal),
    AccessClass(String),
    StoreField(usize),
    StoreTemp(usize),
    /// Send a message to an object
    /// The first usize is the number of arguments to send
    /// The second usize is the index of the message to send
    SendMsg(usize, String),
    SendSuperMsg(usize, String),
    SpecialInstruction(SpecialInstruction),
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
            ByteCode::AccessClass(name) => {
                let idx = string_table.add_string(name.clone());
                let mut binary = vec![5];
                binary.extend(idx.to_binary(None));
                binary
            },
            ByteCode::StoreField(idx) => {
                let mut binary = vec![6];
                binary.extend(idx.to_binary(None));
                binary
            },
            ByteCode::StoreTemp(idx) => {
                let mut binary = vec![7];
                binary.extend(idx.to_binary(None));
                binary
            },
            ByteCode::SendMsg(num_args, name) => {
                let idx = string_table.add_string(name.clone());
                let mut binary = vec![8];
                binary.extend(num_args.to_binary(None));
                binary.extend(idx.to_binary(None));
                binary
            },
            ByteCode::SendSuperMsg(num_args, name) => {
                let idx = string_table.add_string(name.clone());
                let mut binary = vec![9];
                binary.extend(num_args.to_binary(None));
                binary.extend(idx.to_binary(None));
                binary
            },
            ByteCode::SpecialInstruction(inst) => {
                let mut binary = vec![10];
                match inst {
                    SpecialInstruction::DupStack => binary.push(0),
                    SpecialInstruction::DiscardStack => binary.push(1),
                    SpecialInstruction::ReturnStack => binary.push(2),
                    SpecialInstruction::Return => binary.push(3),
                }
                binary
            },
        }
    }
}

#[derive(Clone)]
pub enum SpecialInstruction {
    DupStack,
    DiscardStack,
    ReturnStack,
    Return,
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
