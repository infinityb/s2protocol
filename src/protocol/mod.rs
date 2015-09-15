use std::io::{self};
use std::collections::HashMap;
use phf::Map as PhfMap;

pub mod protocol15405;

mod protocol15405_def;
mod protocol16561_def;
mod protocol16605_def;
mod protocol16755_def;
mod protocol16939_def;
mod protocol17266_def;
mod protocol17326_def;
mod protocol18092_def;
mod protocol18468_def;
mod protocol18574_def;
mod protocol19132_def;
mod protocol19458_def;
mod protocol19595_def;
mod protocol19679_def;
mod protocol21029_def;
mod protocol21995_def;
mod protocol22612_def;
mod protocol23260_def;
mod protocol24764_def;
mod protocol24944_def;
mod protocol26490_def;
mod protocol27950_def;
mod protocol28272_def;
mod protocol28667_def;
mod protocol32283_def;
mod protocol34784_def;
mod protocol34835_def;
mod protocol36442_def;

pub type TypeId = u32;

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
        types: PhfMap<u32, (&'static str, TypeId)>,
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
        items: &'static [(&'static str, TypeId, i32)],
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
    fn read_array(&mut self, tid: TypeId) -> Result<Vec<Value>, ()> {
        let length = try!(self.read_int());
        let mut out = Vec::with_capacity(length as usize);
        for _ in 0..length {
            out.push(try!(self.read_instance()));
        }
        Ok(out)
    }

    fn read_instance(&mut self, tid: TypeId) -> Result<Value, ()> {
        unimplemented!();
    }

    fn read_int(&mut self, tid: TypeId) -> Result<i64, ()> {
        unimplemented!();
    }

    fn read_struct(&mut self, tid: TypeId) -> Result<HashMap<&'static str, Value>, ()> {
        unimplemented!();
    }
}

pub struct VersionedDecoder {
    buffer: BitPackedCursor,
    type_info: TypeInfo,
}