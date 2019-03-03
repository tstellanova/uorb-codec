
extern crate bytes;
extern crate byteorder;

use std::io::{ Error, ErrorKind, Read, Result, Write};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

/// Compatible protocol version number, used for framing messages on the wire
pub const UORB_MAGIC_V1: u8 = 0xAA;


pub trait UorbMsgMeta {
    const ENCODED_LEN: usize;
    const MSG_HASH_CODE: u16;
    const MSG_RAW_NAME: &'static str;

    fn get_hash_code(&self) ->u16 { Self::MSG_HASH_CODE}

    /// Create a UorbHeader for this data with the given uORB instance ID
    fn header_for_instance(&self, instance_id: u8) -> UorbHeader {
        UorbHeader {
            version: UORB_MAGIC_V1,
            hash: self.get_hash_code(),
            instance_id: instance_id,
            payload_len: Self::ENCODED_LEN
        }
    }

    /// Generate a uORB header and message pair for this inner data
    fn gen_ready_pair(&self, instance_id: u8) -> (UorbHeader, UorbMessage) {
        let hdr = self.header_for_instance(instance_id);
        let msg = self.wrap();
        (hdr, msg)
    }

    /// serialize this data as bytes
    fn ser(&self) -> Vec<u8>;



    /// upcast this inner data to the corresponding UorbMessage
    fn wrap(&self) -> UorbMessage;

}



// import code generated by parser at build time
pub mod common {
    use crate::UorbMsgMeta;
    use bytes::{Buf, BufMut, Bytes, IntoBuf};

    include!(concat!(env!("OUT_DIR"), "/common.rs"));
}

/// import all the message definitions
pub use self::common::UorbMessage as UorbMessage;


#[derive(Debug, Clone)]
pub struct UorbHeader {
    /// indicates which version of the header we are sending
    pub version: u8,
    /// unique hash of the msg name
    pub hash: u16,
    /// the "instance" of the sensor/entity that sent this
    pub instance_id: u8,
    /// length of the payload in bytes
    pub payload_len: usize,
}



/// Utilities for sending and receiving uORB via a reliable transport


pub fn write_msg<W: Write>(w: &mut W, header: &UorbHeader, data: &UorbMessage) -> Result<()> {
    let payload = data.ser();

    w.write_u8(header.version)?;
    w.write_u16::<BigEndian>(header.hash)?;
    w.write_u8(header.instance_id)?;
    w.write_u16::<BigEndian>(payload.len() as u16)?;

    w.write_all(&payload[..])?;

    Ok(())
}


pub fn read_msg<R: Read>(r: &mut R) -> Result<(UorbHeader, UorbMessage)> {

    loop {
        // search for the magic framing value indicating start of the message
        if r.read_u8()? != UORB_MAGIC_V1 {
            continue;
        }
        let hash_val:u16 = r.read_u16::<BigEndian>()?;
        let instanced_id = r.read_u8()?;
        let payload_len:usize =  r.read_u16::<BigEndian>()? as usize;

        let header = UorbHeader {
            version: UORB_MAGIC_V1,
            hash: hash_val,
            instance_id: instanced_id,
            payload_len: payload_len,
        };

        //TODO verify that payload size will never exceed this
        let mut payload_buf = [0; 255];
        //println!("payload_len: {}", payload_len);
        let payload = &mut payload_buf[..payload_len.into()];
        r.read_exact(payload)?;

        //println!("parse {} len {}", header.hash, payload_len);
        if let Some(msg) = UorbMessage::parse(header.hash, payload) {
            return Ok((header, msg));
        }
        else {
            let err = Error::new(ErrorKind::InvalidInput,
                                     format!("msg hash: {}",  header.hash));
            return Err(err);
        }
    }

}






