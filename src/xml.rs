extern crate uuid;
extern crate xml;
extern crate rustc_serialize;

use self::xml::writer::*;
use self::xml::writer::events::*;
use self::xml::common::XmlVersion;
use self::xml::namespace::Namespace;
use self::xml::name::Name;
use self::rustc_serialize::Encodable;
use self::rustc_serialize::Encoder;
use std::{char, f64, io, str, string};
use std::io::Write;

pub type EncodeResult<T> = Result<T, EncoderError>;
#[derive(Copy, Debug)]
pub enum EncoderError {
    NastyError,
}
impl Clone for EncoderError {
    fn clone(&self) -> Self { *self }
}

pub fn write<T: Encodable, W : Write>(object: &T, write : W) -> Result<&str, &str> {
    let mut encoder = XmlWriter::new(write);
    let result = {
        object.encode(&mut encoder)
    };
    match result {
        Ok(val) => Ok("success"),
        Err(err) => Err("error")
    }
}

macro_rules! write_element {
    ($enc:ident,$e:expr) => {
        {
            panic!("not implemented");
        }
    }
}

enum EncodingFormat {
    Compact,
    Pretty {
        curr_indent: u32,
        indent: u32
    }
}

struct XmlWriter<W> {
    eventWriter : EventWriter<W>,
}

impl<W : Write> XmlWriter<W> {
    pub fn new(writer : W) -> XmlWriter<W> {
        let config = EmitterConfig::new().write_document_declaration(false);
        let mut eventWriter = EventWriter::new_with_config(writer, config);
        XmlWriter {
            eventWriter : eventWriter
        }
    }

    fn write_element(&mut self, s : &str) -> EncodeResult<()> {
        panic!("not implemented");
    }
}
impl<W : Write> Encoder for XmlWriter<W> {
    type Error = EncoderError;

    fn emit_nil(&mut self) -> EncodeResult<()> {
        panic!("not implemented");
    }

    fn emit_usize(&mut self, v: usize) -> EncodeResult<()> { write_element!(self, v) }
    fn emit_u64(&mut self, v: u64) -> EncodeResult<()> { write_element!(self, v) }
    fn emit_u32(&mut self, v: u32) -> EncodeResult<()> { write_element!(self, v) }
    fn emit_u16(&mut self, v: u16) -> EncodeResult<()> { write_element!(self, v) }
    fn emit_u8(&mut self, v: u8) -> EncodeResult<()> { write_element!(self, v) }

    fn emit_isize(&mut self, v: isize) -> EncodeResult<()> { write_element!(self, v) }
    fn emit_i64(&mut self, v: i64) -> EncodeResult<()> { write_element!(self, v) }
    fn emit_i32(&mut self, v: i32) -> EncodeResult<()> { write_element!(self, v) }
    fn emit_i16(&mut self, v: i16) -> EncodeResult<()> { write_element!(self, v) }
    fn emit_i8(&mut self, v: i8) -> EncodeResult<()> { write_element!(self, v) }

    fn emit_bool(&mut self, v: bool) -> EncodeResult<()> {
        panic!("not implemented");
    }

    fn emit_f64(&mut self, v: f64) -> EncodeResult<()> {
        write_element!(self, fmt_number_or_null(v))
    }
    fn emit_f32(&mut self, v: f32) -> EncodeResult<()> {
        panic!("not implemented");
    }

    fn emit_char(&mut self, v: char) -> EncodeResult<()> {
        panic!("not implemented");
    }
    fn emit_str(&mut self, v: &str) -> EncodeResult<()> {
        match self.eventWriter.write(XmlEvent::Characters(v)) {
            Ok(val) => Ok(val),
            Err(reason) => Err(EncoderError::NastyError)
        }
    }

    fn emit_enum<F>(&mut self, _name: &str, f: F) -> EncodeResult<()> where
        F: FnOnce(&mut Self) -> EncodeResult<()>,
    {
        f(self)
    }

    fn emit_enum_variant<F>(&mut self,
                            name: &str,
                            _id: usize,
                            cnt: usize,
                            f: F)
                            -> EncodeResult<()> where
        F: FnOnce(&mut Self) -> EncodeResult<()>,
    {
        panic!("not implemented");
    }

    fn emit_enum_variant_arg<F>(&mut self, idx: usize, f: F) -> EncodeResult<()> where
        F: FnOnce(&mut Self) -> EncodeResult<()>,
    {
        panic!("not implemented");
    }

    fn emit_enum_struct_variant<F>(&mut self,
                                   name: &str,
                                   id: usize,
                                   cnt: usize,
                                   f: F) -> EncodeResult<()> where
        F: FnOnce(&mut Self) -> EncodeResult<()>,
    {
        panic!("not implemented");
    }

    fn emit_enum_struct_variant_field<F>(&mut self,
                                         _: &str,
                                         idx: usize,
                                         f: F) -> EncodeResult<()> where
        F: FnOnce(&mut Self) -> EncodeResult<()>,
    {
        panic!("not implemented");
    }


    fn emit_struct<F>(&mut self, x: &str, len: usize, f: F) -> EncodeResult<()> where
        F: FnOnce(&mut Self) -> EncodeResult<()>,
    {
        //println!("struct: {} -> {}", x, len);
        f(self)
    }

    fn emit_struct_field<F>(&mut self, field: &str, idx: usize, f: F) -> EncodeResult<()> where
        F: FnOnce(&mut Self) -> EncodeResult<()>,
    {
        let name = Name {
            local_name : field,
            namespace : None,
            prefix : None
        };
        self.eventWriter.write(XmlEvent::StartElement {
            name : name,
            attributes : vec![],
            namespace : &Namespace::empty()
        });
        let r = f(self);
        self.eventWriter.write(XmlEvent::EndElement {
            name : name
        });
        r
    }

    fn emit_tuple<F>(&mut self, len: usize, f: F) -> EncodeResult<()> where
        F: FnOnce(&mut Self) -> EncodeResult<()>,
    {
        panic!("not implemented");
    }
    fn emit_tuple_arg<F>(&mut self, idx: usize, f: F) -> EncodeResult<()> where
        F: FnOnce(&mut Self) -> EncodeResult<()>,
    {
        panic!("not implemented");
    }

    fn emit_tuple_struct<F>(&mut self, _: &str, len: usize, f: F) -> EncodeResult<()> where
        F: FnOnce(&mut Self) -> EncodeResult<()>,
    {
        panic!("not implemented");
    }
    fn emit_tuple_struct_arg<F>(&mut self, idx: usize, f: F) -> EncodeResult<()> where
        F: FnOnce(&mut Self) -> EncodeResult<()>,
    {
        panic!("not implemented");
    }

    fn emit_option<F>(&mut self, f: F) -> EncodeResult<()> where
        F: FnOnce(&mut Self) -> EncodeResult<()>,
    {
        panic!("not implemented");
    }
    fn emit_option_none(&mut self) -> EncodeResult<()> {
        panic!("not implemented");
    }
    fn emit_option_some<F>(&mut self, f: F) -> EncodeResult<()> where
        F: FnOnce(&mut Self) -> EncodeResult<()>,
    {
        panic!("not implemented");
    }

    fn emit_seq<F>(&mut self, len: usize, f: F) -> EncodeResult<()> where
        F: FnOnce(&mut Self) -> EncodeResult<()>,
    {
        panic!("not implemented");
    }

    fn emit_seq_elt<F>(&mut self, idx: usize, f: F) -> EncodeResult<()> where
        F: FnOnce(&mut Self) -> EncodeResult<()>,
    {
        panic!("not implemented");
    }

    fn emit_map<F>(&mut self, len: usize, f: F) -> EncodeResult<()> where
        F: FnOnce(&mut Self) -> EncodeResult<()>,
    {
        panic!("not implemented");
    }

    fn emit_map_elt_key<F>(&mut self, idx: usize, f: F) -> EncodeResult<()> where
        F: FnOnce(&mut Self) -> EncodeResult<()>,
    {
        panic!("not implemented");
    }

    fn emit_map_elt_val<F>(&mut self, _idx: usize, f: F) -> EncodeResult<()> where
        F: FnOnce(&mut Self) -> EncodeResult<()>,
    {
        panic!("not implemented");
    }
}

