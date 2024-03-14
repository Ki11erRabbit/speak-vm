//! The binary format is as follows:
//! header: "SPK" version (u8, u8, u8)
//! class_table: length (u64), [class_table_entry]
//! string_table: length (u64), [string_table_entry]
//! block_table: length (u64), [block_table_entry]
//! 
//! class_table_entry: name_index (u64), flag (u8), parent_index (?u64), method_count (u64), [method_entry], override_count (u64), [override_entry]
//! method_entry: name_index (u64), bytecode_entry
//! bytecode_entry: length (u64), [bytecode]
//! override_entry: length (u64), depth (u64), [method_entry]
//! string_table_entry: length (u64), string (utf-8)
//! block_table_entry: length (u64), [bytecode_entry]

use std::collections::{BTreeMap, HashMap};
use std::cell::RefCell;
use std::sync::Arc;

use nom::{character, number, IResult, Parser, error::Error, multi, bytes, Finish};

use crate::object::{Class, Method, VTable};
use crate::vm::bytecode::ByteCode;


pub fn binary_data_to_binary<'a>(input: &'a [u8]) -> Result<Binary,Error<&[u8]>> {
    let binary = parse_binary(input).finish();
    match binary {
        Ok((_, binary)) => Ok(binary.into_binary()),
        Err(err) => Err(err)
    }
}


fn parse_binary(input: &[u8]) -> IResult<&[u8], ProtoBinary> {
    let (input, _) = parse_header(input)?;
    let (input, class_table) = parse_class_table(input)?;
    let (input, string_table) = parse_string_table(input)?;
    let (input, block_table) = parse_block_table(input)?;
    Ok((input, ProtoBinary { class_table, string_table, block_table }))
}


fn parse_header(input: &[u8]) -> IResult<&[u8],()> {
    let (input, _) = character::complete::char('S')(input)?;
    let (input, _) = character::complete::char('P')(input)?;
    let (input, _) = character::complete::char('K')(input)?;
    let (input, _) = number::complete::u8(input)?;
    let (input, _) = number::complete::u8(input)?;
    let (input, _) = number::complete::u8(input)?;
    Ok((input, ()))
}

fn parse_class_table(input: &[u8]) -> IResult<&[u8], ProtoClassTable> {
    let (input, length) = number::complete::le_u64(input)?;
    let (input, classes) = multi::count(parse_class(), length as usize)(input)?;
    Ok((input, ProtoClassTable { classes }))
}

fn parse_class<'a>() -> impl Parser<&'a [u8], (usize, ProtoClass), Error<&'a [u8]>> {
    |input| {
        let (input, name_index) = number::complete::le_u64(input)?;
        let (input, flag) = number::complete::u8(input)?;
        let (input, parent_index) = if flag != 0 {
            let (input, number) = number::complete::le_u64(input)?;
            (input, Some(number))
        } else {
            (input, None)
        };
        let (input, method_count) = number::complete::le_u64(input)?;
        let (input, methods) = multi::count(parse_method(), method_count as usize)(input)?;
        let (input, override_count) = number::complete::le_u64(input)?;
        let (input, overrides) = multi::count(parse_override, override_count as usize)(input)?;
        Ok((input, (name_index as usize, ProtoClass { parent: parent_index.map(|x| x as usize), methods, overrides })))
    }
}

fn parse_method<'a>() -> impl Parser<&'a [u8], (usize, Vec<ProtoByteCode>), Error<&'a [u8]>> {
    |input| {
        let (input, name_index) = number::complete::le_u64(input)?;
        let (input, bytecode) = parse_bytecode(input)?;
        Ok((input, (name_index as usize, bytecode)))
    }
}

fn parse_override(input: &[u8]) -> IResult<&[u8], (usize, Vec<(usize, Vec<ProtoByteCode>)>)> {
    let (input, length) = number::complete::le_u64(input)?;
    let (input, depth) = number::complete::le_u64(input)?;
    let (input, methods) = multi::count(parse_method(), length as usize)(input)?;
    Ok((input, (depth as usize, methods)))
}

fn parse_string_table(input: &[u8]) -> IResult<&[u8], StringTable> {
    let (input, length) = number::complete::le_u64(input)?;
    let (input, strings) = multi::count(parse_string_entry, length as usize)(input)?;
    Ok((input, StringTable { strings: strings.into_iter().collect(), strings_to_idx: HashMap::new() }))
}

fn parse_string_entry(input: &[u8]) -> IResult<&[u8], (usize, String)> {
    let (input, length) = number::complete::le_u64(input)?;
    let (input, string) = bytes::complete::take(length)(input)?;
    Ok((input, (length as usize, String::from_utf8_lossy(string).to_string())))
}

fn parse_block_table(input: &[u8]) -> IResult<&[u8], ProtoBlockTable> {
    let (input, length) = number::complete::le_u64(input)?;
    let (input, blocks) = multi::count(parse_bytecode, length as usize)(input)?;
    Ok((input, ProtoBlockTable { blocks: blocks.into_iter().enumerate().collect() }))
}

fn parse_bytecode(input: &[u8]) -> IResult<&[u8], Vec<ProtoByteCode>> {
    let (input, length) = number::complete::le_u64(input)?;
    let (input, bytecode) = multi::count(parse_bytecode_entry, length as usize)(input)?;
    Ok((input, bytecode))
}

fn parse_bytecode_entry(input: &[u8]) -> IResult<&[u8], ProtoByteCode> {
    let (input, byte) = number::complete::u8(input)?;
    match byte {
        0 => Ok((input, ProtoByteCode::Halt)),
        1 => Ok((input, ProtoByteCode::NoOp)),
        2 => {
            let (input, idx) = number::complete::le_u64(input)?;
            Ok((input, ProtoByteCode::AccessField(idx as usize)))
        }
        3 => {
            let (input, idx) = number::complete::le_u64(input)?;
            Ok((input, ProtoByteCode::AccessTemp(idx as usize)))
        }
        4 => {
            let (input, lit) = parse_literal(input)?;
            Ok((input, ProtoByteCode::PushLiteral(lit)))
        }
        5 => {
            let (input, idx) = number::complete::le_u64(input)?;
            Ok((input, ProtoByteCode::StoreField(idx as usize)))
        }
        6 => {
            let (input, idx) = number::complete::le_u64(input)?;
            Ok((input, ProtoByteCode::StoreTemp(idx as usize)))
        }
        7 => {
            let (input, arg) = number::complete::le_u64(input)?;
            let (input, msg) = number::complete::le_u64(input)?;
            Ok((input, ProtoByteCode::SendMsg(arg as usize, msg as usize)))
        }
        8 => {
            let (input, arg) = number::complete::le_u64(input)?;
            let (input, msg) = number::complete::le_u64(input)?;
            Ok((input, ProtoByteCode::SendSuperMsg(arg as usize, msg as usize)))
        }
        9 => {
            let (input, instr) = parse_special_instruction(input)?;
            Ok((input, ProtoByteCode::SpecialInstruction(instr)))
        }
        10 => {
            let (input, frame) = number::complete::le_u64(input)?;
            let (input, idx) = number::complete::le_u64(input)?;
            Ok((input, ProtoByteCode::GetStack(frame as usize, idx as usize)))
        }
        _ => unreachable!("Unknown bytecode")
    }
}

fn parse_literal(input: &[u8]) -> IResult<&[u8], ProtoLiteral> {
    let (input, byte) = number::complete::u8(input)?;
    match byte {
        0 => {
            let (input, idx) = number::complete::le_u64(input)?;
            Ok((input, ProtoLiteral::String(idx as usize)))
        }
        1 => {
            let (input, byte) = number::complete::i8(input)?;
            Ok((input, ProtoLiteral::I8(byte)))
        }
        2 => {
            let (input, byte) = number::complete::u8(input)?;
            Ok((input, ProtoLiteral::U8(byte)))
        }
        3 => {
            let (input, byte) = number::complete::le_i16(input)?;
            Ok((input, ProtoLiteral::I16(byte)))
        }
        4 => {
            let (input, byte) = number::complete::le_u16(input)?;
            Ok((input, ProtoLiteral::U16(byte)))
        }
        5 => {
            let (input, byte) = number::complete::le_i32(input)?;
            Ok((input, ProtoLiteral::I32(byte)))
        }
        6 => {
            let (input, byte) = number::complete::le_u32(input)?;
            Ok((input, ProtoLiteral::U32(byte)))
        }
        7 => {
            let (input, byte) = number::complete::le_i64(input)?;
            Ok((input, ProtoLiteral::I64(byte)))
        }
        8 => {
            let (input, byte) = number::complete::le_u64(input)?;
            Ok((input, ProtoLiteral::U64(byte)))
        }
        9 => {
            let (input, byte) = number::complete::le_f32(input)?;
            Ok((input, ProtoLiteral::F32(byte)))
        }
        10 => {
            let (input, byte) = number::complete::le_f64(input)?;
            Ok((input, ProtoLiteral::F64(byte)))
        }
        11 => {
            let (input, byte) = number::complete::u8(input)?;
            Ok((input, ProtoLiteral::Boolean(byte != 0)))
        }
        12 => Ok((input, ProtoLiteral::Nil)),
        13 => {
            let (input, byte) = number::complete::le_u64(input)?;
            Ok((input, ProtoLiteral::ByteCode(byte as usize)))
        }
        _ => unreachable!("Unknown literal")
    }
}

fn parse_special_instruction(input: &[u8]) -> IResult<&[u8], ProtoSpecialInstruction> {
    let (input, byte) = number::complete::u8(input)?;
    match byte {
        0 => Ok((input, ProtoSpecialInstruction::DupStack)),
        1 => Ok((input, ProtoSpecialInstruction::DiscardStack)),
        2 => Ok((input, ProtoSpecialInstruction::ReturnStack)),
        3 => Ok((input, ProtoSpecialInstruction::Return)),
        4 => {
            let (input, idx) = number::complete::le_u64(input)?;
            Ok((input, ProtoSpecialInstruction::PopTrueSkip(idx as usize)))
        }
        5 => {
            let (input, idx) = number::complete::le_u64(input)?;
            Ok((input, ProtoSpecialInstruction::PopFalseSkip(idx as usize)))
        }
        6 => {
            let (input, idx) = number::complete::le_u64(input)?;
            Ok((input, ProtoSpecialInstruction::PopTrueBackSkip(idx as usize)))
        }
        7 => {
            let (input, idx) = number::complete::le_u64(input)?;
            Ok((input, ProtoSpecialInstruction::PopFalseBackSkip(idx as usize)))
        }
        8 => {
            let (input, idx) = number::complete::le_u64(input)?;
            Ok((input, ProtoSpecialInstruction::Skip(idx as usize)))
        }
        9 => {
            let (input, idx) = number::complete::le_u64(input)?;
            Ok((input, ProtoSpecialInstruction::BackSkip(idx as usize)))
        }
        _ => unreachable!("Unknown special instruction")
    }
}

pub struct ProtoBinary {
    class_table: ProtoClassTable,
    string_table: StringTable,
    block_table: ProtoBlockTable,
}

impl ProtoBinary {
    pub fn into_binary(self) -> Binary {
        let block_table = self.block_table.into_block_table(&self.string_table);
        let class_table = self.class_table.into_class_table(&self.string_table, &block_table);
        let string_table = RefCell::new(self.string_table);
        Binary { class_table, string_table, block_table }
    }

    pub fn to_binary(self) -> Vec<u8> {
        let mut binary = vec![];
        binary.extend_from_slice(&[0x53, 0,50, 0x4b]); // SPK
        binary.extend_from_slice(&[0,0,1]); // version
        binary.extend(self.class_table.to_binary(None));
        binary.extend(self.string_table.to_binary(None));
        binary.extend(self.block_table.to_binary(None));
        binary
    }
}


pub struct ProtoClassTable {
    classes: Vec<(usize, ProtoClass)>,
}

impl ProtoClassTable {
    pub fn into_class_table(self, string_table: &StringTable, block_table: &BlockTable) -> ClassTable {
        let classes = self.classes.into_iter().map(|(idx, class)| {
            let name = string_table.strings.get(&idx).expect("Expected string").clone();
            (name, class.into_class(string_table, block_table))
        }).collect();
        ClassTable { classes }
    }
}
impl ToBinary for ProtoClassTable {
    fn to_binary(&self, _: Option<&mut StringTable>) -> Vec<u8> {
        let mut binary = vec![];
        binary.extend_from_slice(self.classes.len().to_binary(None).as_slice());
        for (idx, class) in self.classes.iter() {
            binary.extend_from_slice(idx.to_binary(None).as_slice());
            binary.extend_from_slice(class.to_binary(None).as_slice());
        }
        binary
    }
}

pub struct ProtoClass {
    parent: Option<usize>,
    methods: Vec<(usize, Vec<ProtoByteCode>)>,
    overrides: Vec<(usize, Vec<(usize, Vec<ProtoByteCode>)>)>,
}

impl ProtoClass {
    pub fn into_class(self, string_table: &StringTable, block_table: &BlockTable) -> Class {
        let parent = if let Some(idx) = self.parent {
            Some(string_table.strings.get(&idx).expect("Expected string").as_str())
        } else {
            None
        };
        let mut methods = HashMap::new();
        for (idx, bytecode) in self.methods {
            let name = string_table.strings.get(&idx).expect("Expected string").clone();
            let bytecode = bytecode.into_iter().map(|bytecode| bytecode.into_bytecode(string_table, block_table)).collect::<Vec<ByteCode>>();
            let block = crate::object::create_block(bytecode);
            methods.insert(name, Arc::new(Method::BytecodeMethod { block }));
        }
        let mut overrides = BTreeMap::new();
        for (depth, methods) in self.overrides {
            let mut vtable: HashMap<String, Arc<Method>> = HashMap::new();
            for (idx, bytecode) in methods {
                let name = string_table.strings.get(&idx).expect("Expected string").clone();
                let bytecode = bytecode.into_iter().map(|bytecode| bytecode.into_bytecode(string_table, block_table)).collect();
                let block = crate::object::create_block(bytecode);
                vtable.insert(name, Arc::new(Method::BytecodeMethod { block }));
            }
            overrides.insert(depth, VTable::new(vtable));
        }
        let mut overrides_vec = Vec::new();
        for (_, vtable) in overrides.into_iter().rev() {
            overrides_vec.push(vtable);
        }
        Class::new(parent, VTable::new(methods), overrides_vec)
    }
}

impl ToBinary for ProtoClass {
    fn to_binary(&self, _: Option<&mut StringTable>) -> Vec<u8> {
        let mut binary = vec![];
        if let Some(parent) = self.parent {
            binary.extend_from_slice(&[1]);
            binary.extend_from_slice(parent.to_binary(None).as_slice());
        } else {
            binary.extend_from_slice(&[0]);
        }
        binary.extend_from_slice(self.methods.len().to_binary(None).as_slice());
        for (idx, bytecode) in &self.methods {
            binary.extend_from_slice(idx.to_binary(None).as_slice());
            binary.extend_from_slice(bytecode.len().to_binary(None).as_slice());
            for byte in bytecode.iter() {
            binary.extend_from_slice(byte.to_binary(None).as_slice());
            }
        }
        binary.extend_from_slice(self.overrides.len().to_binary(None).as_slice());
        for (depth, methods) in &self.overrides {
            binary.extend_from_slice(methods.len().to_binary(None).as_slice());
            binary.extend_from_slice(depth.to_binary(None).as_slice());
            for (idx, bytecode) in methods {
                binary.extend_from_slice(idx.to_binary(None).as_slice());
                binary.extend_from_slice(bytecode.len().to_binary(None).as_slice());
                for byte in bytecode.iter() {
                    binary.extend_from_slice(byte.to_binary(None).as_slice());
                }
            }
        }
        binary

    }
}

pub struct ProtoBlockTable {
    blocks: BTreeMap<usize, Vec<ProtoByteCode>>,
}

impl ProtoBlockTable {
    pub fn into_block_table(self, string_table: &StringTable) -> BlockTable {
        let mut block_table = BlockTable { blocks: BTreeMap::new() };
        let _ = self.blocks.into_iter().map(|(idx, bytecode)| {
            let bytecode = bytecode.into_iter().map(|bytecode| bytecode.into_bytecode(string_table, &block_table)).collect();
            block_table.blocks.insert(idx, bytecode);
        });
        block_table
    }
}

impl ToBinary for ProtoBlockTable {
    fn to_binary(&self, string_table: Option<&mut StringTable>) -> Vec<u8> {
        let string_table = string_table.expect("ProtoBlockTable::to_binary called without a StringTable");
        let mut binary = vec![];
        binary.extend_from_slice(self.blocks.len().to_binary(None).as_slice());
        for (idx, block) in self.blocks.iter() {
            binary.extend_from_slice(idx.to_binary(None).as_slice());
            binary.extend_from_slice(block.len().to_binary(None).as_slice());
            for byte in block.iter() {
                binary.extend_from_slice(byte.to_binary(Some(string_table)).as_slice());
            }
        }
        binary
    }
}

pub enum ProtoByteCode {
    Halt,
    NoOp,
    AccessField(usize),
    AccessTemp(usize),
    PushLiteral(ProtoLiteral),
    StoreField(usize),
    StoreTemp(usize),
    SendMsg(usize, usize),
    SendSuperMsg(usize, usize),
    SpecialInstruction(ProtoSpecialInstruction),
    GetStack(usize, usize),

}

impl ProtoByteCode {
    pub fn into_bytecode(self, string_table: &StringTable, block_table: &BlockTable) -> ByteCode {
        match self {
            ProtoByteCode::Halt => ByteCode::Halt,
            ProtoByteCode::NoOp => ByteCode::NoOp,
            ProtoByteCode::AccessField(idx) => ByteCode::AccessField(idx),
            ProtoByteCode::AccessTemp(idx) => ByteCode::AccessTemp(idx),
            ProtoByteCode::PushLiteral(lit) => ByteCode::PushLiteral(lit.into_literal(string_table, block_table)),
            ProtoByteCode::StoreField(idx) => ByteCode::StoreField(idx),
            ProtoByteCode::StoreTemp(idx) => ByteCode::StoreTemp(idx),
            ProtoByteCode::SendMsg(arg, msg) => ByteCode::SendMsg(arg, string_table.strings.get(&msg).expect("Expected string").clone()),
            ProtoByteCode::SendSuperMsg(arg, msg) => ByteCode::SendSuperMsg(arg, string_table.strings.get(&msg).expect("Expected string").clone()),
            ProtoByteCode::SpecialInstruction(inst) => ByteCode::SpecialInstruction(inst.into()),
            ProtoByteCode::GetStack(frame, idx) => ByteCode::GetStack(frame, idx),
        }
    }
}

impl ToBinary for ProtoByteCode {
    fn to_binary(&self, _: Option<&mut StringTable>) -> Vec<u8> {
        let mut binary = vec![];
        match self {
            ProtoByteCode::Halt => binary.push(0),
            ProtoByteCode::NoOp => binary.push(1),
            ProtoByteCode::AccessField(idx) => {
                binary.push(2);
                binary.extend_from_slice(idx.to_binary(None).as_slice());
            }
            ProtoByteCode::AccessTemp(idx) => {
                binary.push(3);
                binary.extend_from_slice(idx.to_binary(None).as_slice());
            }
            ProtoByteCode::PushLiteral(lit) => {
                binary.push(4);
                binary.extend_from_slice(lit.to_binary(None).as_slice());
            }
            ProtoByteCode::StoreField(idx) => {
                binary.push(5);
                binary.extend_from_slice(idx.to_binary(None).as_slice());
            }
            ProtoByteCode::StoreTemp(idx) => {
                binary.push(6);
                binary.extend_from_slice(idx.to_binary(None).as_slice());
            }
            ProtoByteCode::SendMsg(arg, msg) => {
                binary.push(7);
                binary.extend_from_slice(arg.to_binary(None).as_slice());
                binary.extend_from_slice(msg.to_binary(None).as_slice());
            }
            ProtoByteCode::SendSuperMsg(arg, msg) => {
                binary.push(8);
                binary.extend_from_slice(arg.to_binary(None).as_slice());
                binary.extend_from_slice(msg.to_binary(None).as_slice());
            }
            ProtoByteCode::SpecialInstruction(inst) => {
                binary.push(9);
                binary.extend_from_slice(inst.to_binary(None).as_slice());
            }
            ProtoByteCode::GetStack(frame, idx) => {
                binary.push(10);
                binary.extend_from_slice(frame.to_binary(None).as_slice());
                binary.extend_from_slice(idx.to_binary(None).as_slice());
            }
        }
        binary
    }
}
   
pub enum ProtoSpecialInstruction {
    DupStack,
    DiscardStack,
    ReturnStack,
    Return,
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

impl Into<crate::vm::bytecode::SpecialInstruction> for ProtoSpecialInstruction {
    fn into(self) -> crate::vm::bytecode::SpecialInstruction {
        match self {
            ProtoSpecialInstruction::DupStack => crate::vm::bytecode::SpecialInstruction::DupStack,
            ProtoSpecialInstruction::DiscardStack => crate::vm::bytecode::SpecialInstruction::DiscardStack,
            ProtoSpecialInstruction::ReturnStack => crate::vm::bytecode::SpecialInstruction::ReturnStack,
            ProtoSpecialInstruction::Return => crate::vm::bytecode::SpecialInstruction::Return,
            ProtoSpecialInstruction::PopTrueSkip(n) => crate::vm::bytecode::SpecialInstruction::PopTrueSkip(n),
            ProtoSpecialInstruction::PopFalseSkip(n) => crate::vm::bytecode::SpecialInstruction::PopFalseSkip(n),
            ProtoSpecialInstruction::PopTrueBackSkip(n) => crate::vm::bytecode::SpecialInstruction::PopTrueBackSkip(n),
            ProtoSpecialInstruction::PopFalseBackSkip(n) => crate::vm::bytecode::SpecialInstruction::PopFalseBackSkip(n),
            ProtoSpecialInstruction::Skip(n) => crate::vm::bytecode::SpecialInstruction::Skip(n),
            ProtoSpecialInstruction::BackSkip(n) => crate::vm::bytecode::SpecialInstruction::BackSkip(n),
                    
        }
    }
}

impl ToBinary for ProtoSpecialInstruction {
    fn to_binary(&self, _: Option<&mut StringTable>) -> Vec<u8> {
        match self {
            ProtoSpecialInstruction::DupStack => vec![0],
            ProtoSpecialInstruction::DiscardStack => vec![1],
            ProtoSpecialInstruction::ReturnStack => vec![2],
            ProtoSpecialInstruction::Return => vec![3],
            ProtoSpecialInstruction::PopTrueSkip(n) => {
                let mut binary = vec![4];
                binary.extend_from_slice(n.to_binary(None).as_slice());
                binary
            }
            ProtoSpecialInstruction::PopFalseSkip(n) => {
                let mut binary = vec![5];
                binary.extend_from_slice(n.to_binary(None).as_slice());
                binary
            }
            ProtoSpecialInstruction::PopTrueBackSkip(n) => {
                let mut binary = vec![6];
                binary.extend_from_slice(n.to_binary(None).as_slice());
                binary
            }
            ProtoSpecialInstruction::PopFalseBackSkip(n) => {
                let mut binary = vec![7];
                binary.extend_from_slice(n.to_binary(None).as_slice());
                binary
            }
            ProtoSpecialInstruction::Skip(n) => {
                let mut binary = vec![8];
                binary.extend_from_slice(n.to_binary(None).as_slice());
                binary
            }
            ProtoSpecialInstruction::BackSkip(n) => {
                let mut binary = vec![9];
                binary.extend_from_slice(n.to_binary(None).as_slice());
                binary
            }
        }
    }
}


pub enum ProtoLiteral {
    String(usize),
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
    ByteCode(usize),
}

impl ProtoLiteral {
    pub fn into_literal(self, string_table: &StringTable, block_table: &BlockTable) -> crate::vm::bytecode::Literal {
        match self {
            ProtoLiteral::String(idx) => crate::vm::bytecode::Literal::String(string_table.strings.get(&idx).expect("Expected string").clone()),
            ProtoLiteral::I8(byte) => crate::vm::bytecode::Literal::I8(byte),
            ProtoLiteral::U8(byte) => crate::vm::bytecode::Literal::U8(byte),
            ProtoLiteral::I16(byte) => crate::vm::bytecode::Literal::I16(byte),
            ProtoLiteral::U16(byte) => crate::vm::bytecode::Literal::U16(byte),
            ProtoLiteral::I32(byte) => crate::vm::bytecode::Literal::I32(byte),
            ProtoLiteral::U32(byte) => crate::vm::bytecode::Literal::U32(byte),
            ProtoLiteral::I64(byte) => crate::vm::bytecode::Literal::I64(byte),
            ProtoLiteral::U64(byte) => crate::vm::bytecode::Literal::U64(byte),
            ProtoLiteral::F32(byte) => crate::vm::bytecode::Literal::F32(byte),
            ProtoLiteral::F64(byte) => crate::vm::bytecode::Literal::F64(byte),
            ProtoLiteral::Boolean(byte) => crate::vm::bytecode::Literal::Boolean(byte),
            ProtoLiteral::Nil => crate::vm::bytecode::Literal::Nil,
            ProtoLiteral::ByteCode(byte) => crate::vm::bytecode::Literal::ByteCode(block_table.blocks.get(&byte).expect("Expected block").clone()),
        }
    }
}

impl ToBinary for ProtoLiteral {
    fn to_binary(&self, _string_table: Option<&mut StringTable>) -> Vec<u8> {
        let mut binary = vec![];
        match self {
            ProtoLiteral::String(idx) => {
                binary.push(0);
                binary.extend_from_slice(idx.to_binary(None).as_slice());
            }
            ProtoLiteral::I8(byte) => {
                binary.push(1);
                binary.extend_from_slice(byte.to_le_bytes().to_vec().as_slice());
            }
            ProtoLiteral::U8(byte) => {
                binary.push(2);
                binary.extend_from_slice(byte.to_le_bytes().to_vec().as_slice());
            }
            ProtoLiteral::I16(byte) => {
                binary.push(3);
                binary.extend_from_slice(byte.to_le_bytes().to_vec().as_slice());
            }
            ProtoLiteral::U16(byte) => {
                binary.push(4);
                binary.extend_from_slice(byte.to_le_bytes().to_vec().as_slice());
            }
            ProtoLiteral::I32(byte) => {
                binary.push(5);
                binary.extend_from_slice(byte.to_le_bytes().to_vec().as_slice());
            }
            ProtoLiteral::U32(byte) => {
                binary.push(6);
                binary.extend_from_slice(byte.to_le_bytes().to_vec().as_slice());
            }
            ProtoLiteral::I64(byte) => {
                binary.push(7);
                binary.extend_from_slice(byte.to_le_bytes().to_vec().as_slice());
            }
            ProtoLiteral::U64(byte) => {
                binary.push(8);
                binary.extend_from_slice(byte.to_le_bytes().to_vec().as_slice());
            }
            ProtoLiteral::F32(byte) => {
                binary.push(9);
                binary.extend_from_slice(byte.to_le_bytes().to_vec().as_slice());
            }
            ProtoLiteral::F64(byte) => {
                binary.push(10);
                binary.extend_from_slice(byte.to_le_bytes().to_vec().as_slice());
            }
            ProtoLiteral::Boolean(byte) => {
                binary.push(11);
                match byte {
                    true => binary.push(1),
                    false => binary.push(0),
                }
            }
            ProtoLiteral::Nil => binary.push(12),
            ProtoLiteral::ByteCode(byte) => {
                binary.push(13);
                binary.extend_from_slice(byte.to_binary(None).as_slice());
            }
        }
        binary
    }
}

pub struct Binary {
    pub class_table: ClassTable,
    string_table: RefCell<StringTable>,
    block_table: BlockTable,
}

impl Binary {
    pub fn to_binary(&self) -> Vec<u8> {
        let mut binary = vec![];
        binary.extend_from_slice(&[0x53, 0,50, 0x4b]); // SPK
        binary.extend_from_slice(&[0,0,1]); // version
        binary.extend(self.class_table.to_binary(Some(&mut self.string_table.borrow_mut())));
        binary.extend(self.string_table.borrow().to_binary(None));
        binary.extend(self.block_table.to_binary(Some(&mut self.string_table.borrow_mut())));
        binary
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Class)> {
        self.class_table.iter()
    }
    pub fn into_iter(self) -> impl Iterator<Item = (String, Class)> {
        self.class_table.classes.into_iter()
    }
}



pub trait ToBinary {
    fn to_binary(&self, string_table: Option<&mut StringTable>) -> Vec<u8>;
}

pub struct ClassTable {
    classes: HashMap<String, Class>,
}

impl ClassTable {
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Class)> {
        self.classes.iter()
    }
}

impl ToBinary for ClassTable {
    fn to_binary(&self, string_table: Option<&mut StringTable>) -> Vec<u8> {
        let mut binary = vec![];
        let string_table = string_table.expect("ClassTable::to_binary called without a StringTable");
        binary.extend_from_slice(self.classes.len().to_binary(None).as_slice());
        for name in self.classes.keys() {
            let idx = string_table.add_string(name.clone());
            binary.extend_from_slice(idx.to_binary(None).as_slice());
            self.classes.get(name).unwrap().to_binary(Some(string_table));
        }
        binary
    }
}


pub struct StringTable {
    strings: BTreeMap<usize, String>,
    strings_to_idx: HashMap<String, usize>,
}

impl StringTable {
    pub fn add_string(&mut self, string: String) -> usize {
        let idx = if self.strings_to_idx.contains_key(&string) {
            *self.strings_to_idx.get(&string).unwrap()
        } else {
            self.strings.len()
        };
        self.strings.insert(idx, string.clone());
        self.strings_to_idx.insert(string, idx);
        idx
    }
}

impl ToBinary for StringTable {
    fn to_binary(&self, _: Option<&mut StringTable>) -> Vec<u8> {
        let mut binary = vec![];
        binary.extend_from_slice(self.strings.len().to_binary(None).as_slice());
        for (_, string) in self.strings.iter() {
            binary.extend_from_slice(string.len().to_binary(None).as_slice());
            binary.extend_from_slice(string.as_bytes());
        }
        binary
    }
}

pub struct BlockTable {
    blocks: BTreeMap<usize, Vec<ByteCode>>,
    //block_to_idx: HashMap<Vec<ByteCode>, usize>,
}

impl ToBinary for BlockTable {
    fn to_binary(&self, string_table: Option<&mut StringTable>) -> Vec<u8> {
        let string_table = string_table.expect("BlockTable::to_binary called without a StringTable");
        let mut binary = vec![];
        binary.extend_from_slice(self.blocks.len().to_binary(None).as_slice());
        for (_, block) in self.blocks.iter() {
            binary.extend_from_slice(block.len().to_binary(None).as_slice());
            for byte in block.iter() {
                binary.extend_from_slice(byte.to_binary(Some(string_table)).as_slice());
            }
        }
        binary
    }
}


impl ToBinary for usize {
    fn to_binary(&self, _: Option<&mut StringTable>) -> Vec<u8> {
        (*self as u64).to_le_bytes().to_vec()
    }
}

