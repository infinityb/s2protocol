use std::collections::HashMap;
use super::{TypeInfo, MpqError, BitPackedDecoder};
use byteorder::{BigEndian, ByteOrder};

pub trait InstanceReader {
    type Item: Sized;

    fn read(&self, decoder: &mut BitPackedDecoder) -> Result<Self::Item, MpqError>;
}

pub enum Unknown {}

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

pub struct ArrayReader<Elem> where Elem: InstanceReader {
    element: Elem,
}

impl<Elem> InstanceReader for ArrayReader<Elem> where Elem: InstanceReader {
    type Item = Vec<<Elem as InstanceReader>::Item>;

    fn read(&self, decoder: &mut BitPackedDecoder) -> Result<Vec<Elem::Item>, MpqError> {
        let length = try!(decoder.read_instance(&IntegerReader));
        let mut out: Vec<Elem::Item> = Vec::with_capacity(length as usize);
        for _ in 0..length {
            out.push(try!(decoder.read_instance(&self.element)));
        }
        Ok(out)
    }
}

pub struct FourCc([u8; 4]);

pub struct FourCcReader;

impl InstanceReader for FourCcReader {
    type Item = FourCc;

    fn read(&self, decoder: &mut BitPackedDecoder) -> Result<Self::Item, MpqError> {
        let mut buf = [0; 4];
        assert_eq!(4, try!(decoder.cursor.read_unaligned_bytes(&mut buf[..])));
        Ok(FourCc(buf))
    }
}

pub struct Float32Reader;

impl InstanceReader for Float32Reader {
    type Item = f32;

    fn read(&self, decoder: &mut BitPackedDecoder) -> Result<Self::Item, MpqError> {
        let mut buf = [0; 4];
        assert_eq!(4, try!(decoder.cursor.read_aligned_bytes(&mut buf[..])));
        Ok(BigEndian::read_f32(&buf[..]))
    }
}

pub struct Float64Reader;

impl InstanceReader for Float64Reader {
    type Item = f64;

    fn read(&self, decoder: &mut BitPackedDecoder) -> Result<Self::Item, MpqError> {
        let mut buf = [0; 8];
        assert_eq!(8, try!(decoder.cursor.read_aligned_bytes(&mut buf[..])));
        Ok(BigEndian::read_f64(&buf[..]))
    }
}

pub struct StructReader {
    fields: &'static [super::StructField],
}

impl InstanceReader for StructReader {
    type Item = HashMap<&'static str, Value>;

    fn read(&self, decoder: &mut BitPackedDecoder) -> Result<Self::Item, MpqError> {
        let mut out = HashMap::new();
        for &(name, typeid, tag) in self.fields.iter() {
            // FIXME: can this be typed?
            out.insert(name, try!(decoder.read_value(typeid)));
        }
        Ok(out)
    }
}

impl StructReader {
    pub fn new(fields: &'static [super::StructField]) -> Self {
        StructReader { fields: fields }
    }
}

pub struct IntegerReader;

impl InstanceReader for IntegerReader {
    type Item = i64;

    fn read(&self, decoder: &mut BitPackedDecoder) -> Result<Self::Item, MpqError> {
        unimplemented!();
    }
}
