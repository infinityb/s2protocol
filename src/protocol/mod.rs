use std::io::{self};
use std::collections::HashMap;

mod protocol15405;

type TypeId = u32;

use self::protocol15405::Protocol15405;

pub enum Unknown {}

pub enum InitData {}

pub enum ReplayDetails {}

pub enum ReplayGameEvents {}

pub enum ReplayMessageEvents {}

pub enum ReplayTrackerEvents {}

pub enum ReplayAttributesEvents {}

pub enum Value {
    Array(Vec<Value>),
    BitArray(Unknown),
    Blob(Vec<u8>),
    Bool(bool),
    Choice {
        name: &'static str,
        value: Box<Value>,
    },
    FourCC([u8; 4]),
    Int(i64),
    Null,
    Optional(Option<Box<Value>>),
    Real32(f32),
    Real64(f64),
    Struct(HashMap<&'static str, Value>),
}

pub enum TypeInfo {
    Array {
        bounds: (usize, usize),
        typeid: TypeId,
    },
    BitArray {
        // a TypeInfo::Int prefixed blob, unaligned.
        len_min: i64,
        len_bits: usize
    },
    Blob {
        // a TypeInfo::Int prefixed blob, aligned.
        len_min: i64,
        len_bits: usize
    },
    Bool,
    Choice {
        // a TypeInfo::Int key
        min: i64,
        bits: usize,
        types: Vec<(&'static str, TypeId)>,
    },
    FourCC,
    Int {
        min: i64,
        bits: usize,
    },
    Null,
    Optional {
        typeid: TypeId,
    },
    Real32,
    Real64,
    Struct {
        // name, type, tag
        items: Vec<(&'static str, TypeId, i32)>,
    },
}

pub trait Protocol {
    fn protocol_num(&self) -> u32;

    fn decode_replay_initdata(&self, rdr: &mut io::Read) -> io::Result<InitData>;

    fn decode_replay_details(&self, rdr: &mut io::Read) -> io::Result<ReplayDetails>;

    fn decode_replay_game_events(&self, rdr: &mut io::Read) -> io::Result<ReplayGameEvents>;

    fn decode_replay_message_events(&self, rdr: &mut io::Read) -> io::Result<ReplayMessageEvents>;

    fn decode_replay_tracker_events(&self, rdr: &mut io::Read) -> io::Result<ReplayTrackerEvents>;

    fn decode_replay_attributes_events(&self, rdr: &mut io::Read) -> io::Result<ReplayAttributesEvents>;
}

pub fn get_protocol(base_build: u32) -> Result<Box<Protocol>, ()> {
    match base_build {
        15405 => Ok(Box::new(Protocol15405)),
        // 16561 => Ok(Box::new(Protocol16561)),
        // 16605 => Ok(Box::new(Protocol16605)),
        // 16755 => Ok(Box::new(Protocol16755)),
        // 16939 => Ok(Box::new(Protocol16939)),
        // 17266 => Ok(Box::new(Protocol17266)),
        // 17326 => Ok(Box::new(Protocol17326)),
        // 18092 => Ok(Box::new(Protocol18092)),
        // 18468 => Ok(Box::new(Protocol18468)),
        // 18574 => Ok(Box::new(Protocol18574)),
        // 19132 => Ok(Box::new(Protocol19132)),
        // 19458 => Ok(Box::new(Protocol19458)),
        // 19595 => Ok(Box::new(Protocol19595)),
        // 19679 => Ok(Box::new(Protocol19679)),
        // 21029 => Ok(Box::new(Protocol21029)),
        // 21995 => Ok(Box::new(Protocol21995)),
        // 22612 => Ok(Box::new(Protocol22612)),
        // 23260 => Ok(Box::new(Protocol23260)),
        // 24764 => Ok(Box::new(Protocol24764)),
        // 24944 => Ok(Box::new(Protocol24944)),
        // 26490 => Ok(Box::new(Protocol26490)),
        // 27950 => Ok(Box::new(Protocol27950)),
        // 28272 => Ok(Box::new(Protocol28272)),
        // 28667 => Ok(Box::new(Protocol28667)),
        // 32283 => Ok(Box::new(Protocol32283)),
        // 34784 => Ok(Box::new(Protocol34784)),
        // 34835 => Ok(Box::new(Protocol34835)),
        // 36442 => Ok(Box::new(Protocol36442)),
        _ => Err(())
    }
}

pub struct BitPackedCursor {
    _fields: Unknown,
}

impl BitPackedCursor {
    fn read_array(&mut self) -> Result<Vec<Value>, ()> {
        let length = try!(self.read_int());
        let mut out = Vec::with_capacity(length as usize);
        for _ in 0..length {
            out.push(try!(self.read_instance()));
        }
        Ok(out)
    }

    fn read_instance(&mut self) -> Result<Value, ()> {
        unimplemented!();
    }

    fn read_int(&mut self) -> Result<i64, ()> {
        unimplemented!();
    }

    fn read_struct(&mut self) -> Result<HashMap<&'static str, Value>, ()> {
        unimplemented!();
    }
}

pub struct VersionedDecoder {
    buffer: BitPackedCursor,
    type_info: TypeInfo,
}