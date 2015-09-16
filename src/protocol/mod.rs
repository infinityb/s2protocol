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
    Choice(&'static str, Box<Value>),
    FourCC([u8; 4]),
    Int(i64),
    Null,
    Optional(Option<Box<Value>>),
    Real32(f32),
    Real64(f64),
    Struct(HashMap<&'static str, Value>),
}

pub type ChoiceTypeMap = PhfMap<u32, (&'static str, TypeId)>;

/// name, type, tag
pub type StructField = (&'static str, TypeId, i32);

#[derive(Copy, Clone)]
pub struct IntBounds {
    min: i64,
    bitlen: u8,
}

pub enum TypeInfo {
    Array {
        bounds: IntBounds,
        typeid: TypeId,
    },
    BitArray { len: IntBounds },
    Blob { len: IntBounds },
    Bool,
    Choice {
        bounds: IntBounds,
        types: ChoiceTypeMap,
    },
    FourCC,
    Int { bounds: IntBounds },
    Null,
    Optional {
        typeid: TypeId,
    },
    Real32,
    Real64,
    Struct {
        fields: &'static [StructField],
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

pub enum MpqError {
    Corrupted,
    Truncated,
}

pub struct BitPackedCursor<'a> {
    data: &'a [u8],
    used: u8,
    next: u8,
    nextbits: u8,
    bigendian: bool,
}

impl<'a> BitPackedCursor<'a> {
    fn done(&self) -> bool {
        unimplemented!();
    }

    fn used_bits(&self) -> usize {
        unimplemented!();
    }

    fn byte_align(&mut self) -> () {
        unimplemented!();
    }

    fn read_aligned_bytes(self, len: usize) -> Result<Vec<u8>, MpqError> {
        unimplemented!();
    }
}

struct BitPackedDecoder<'a> {
    cursor: BitPackedCursor<'a>,
    typeinfos: &'static [TypeInfo],
}

impl<'a> BitPackedDecoder<'a> {
    fn read_instance(&mut self, tid: TypeId) -> Result<Value, MpqError> {
        let typeinfo = try!(self.typeinfos
            .get(tid as usize)
            .ok_or(MpqError::Corrupted));
        match *typeinfo {
            TypeInfo::Array { bounds, typeid } => {
                let value = try!(self.read_array(bounds, tid));
                Ok(Value::Array(value))
            }
            TypeInfo::BitArray { len } => {
                let value = try!(self.read_bitarray(len));
                unimplemented!();
            }
            TypeInfo::Blob { len } => {
                let value = try!(self.read_blob(len));
                Ok(Value::Blob(value))
            }
            TypeInfo::Bool=> {
                let value = try!(self.read_bool());
                Ok(Value::Bool(value))
            }
            TypeInfo::Choice { bounds, ref types } => {
                let (name, value) = try!(self.read_choice(bounds, types));
                Ok(Value::Choice(name, Box::new(value)))
            }
            TypeInfo::FourCC => {
                let value = try!(self.read_fourcc());
                unimplemented!();
            }
            TypeInfo::Int { bounds } => {
                let value = try!(self.read_bitarray(bounds));
                unimplemented!();
            }
            TypeInfo::Null => {
                unimplemented!();
            }
            TypeInfo::Optional { typeid } => {
                unimplemented!();
            }
            TypeInfo::Real32 => {
                unimplemented!();
            }
            TypeInfo::Real64 => {
                unimplemented!();
            }
            TypeInfo::Struct { fields } => {
                let value = try!(self.read_struct(fields));
                Ok(Value::Struct(value))
            }
        }
    }

    fn read_array(&mut self, bounds: IntBounds, tid: TypeId) -> Result<Vec<Value>, MpqError> {
        let length = try!(self.read_int(bounds));
        let mut out = Vec::with_capacity(length as usize);
        for _ in 0..length {
            out.push(try!(self.read_instance(tid)));
        }
        Ok(out)
    }

    fn read_bitarray(&self, bounds: IntBounds) -> Result<(), MpqError> {
        unimplemented!();
    }
    
    fn read_blob(&self, bounds: IntBounds) -> Result<Vec<u8>, MpqError> {
        unimplemented!();
    }
    
    fn read_bool(&self) -> Result<bool, MpqError> {
        unimplemented!();
    }
    
    fn read_choice(&self, bounds: IntBounds, fields: &ChoiceTypeMap) -> Result<(&'static str, Value), MpqError> {
        unimplemented!();
    }
    
    fn read_fourcc(&self) -> Result<[u8; 4], MpqError> {
        unimplemented!();
    }
    
    fn read_int(&self, bounds: IntBounds) -> Result<i64, MpqError> {
        unimplemented!();
    }
    
    fn read_null(&self) -> Result<(), MpqError> {
        unimplemented!();
    }
    
    fn read_optional(&self, tid: TypeId) -> Result<Option<Value>, MpqError> {
        unimplemented!();
    }
    
    fn read_real32(&self) -> Result<f32, MpqError> {
        unimplemented!();
    }
    
    fn read_real64(&self) -> Result<f64, MpqError> {
        unimplemented!();
    }
    
    fn read_struct(&self, fields: &[StructField]) -> Result<HashMap<&'static str, Value>, MpqError> {
        unimplemented!();
    }
}

pub struct VersionedDecoder<'a> {
    buffer: BitPackedCursor<'a>,
    type_info: TypeInfo,
}