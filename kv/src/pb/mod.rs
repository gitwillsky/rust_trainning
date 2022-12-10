mod abi;

pub use abi::*;
use bytes::{Bytes, BytesMut};
use prost::Message;

impl Request {
    pub fn new_get(key: String) -> Self {
        Self {
            command: Some(request::Command::Get(RequestGet { key })),
        }
    }

    pub fn new_put(key: String, value: Vec<u8>) -> Self {
        Self {
            command: Some(request::Command::Put(RequestPut { key, value })),
        }
    }
}

impl TryFrom<BytesMut> for Request {
    type Error = prost::DecodeError;

    fn try_from(buf: BytesMut) -> Result<Self, Self::Error> {
        Message::decode(buf)
    }
}

impl From<Request> for Bytes {
    fn from(msg: Request) -> Self {
        let mut buf = BytesMut::new();
        msg.encode(&mut buf).unwrap();
        buf.freeze()
    }
}

impl Response {
    pub fn success(key: String, value: Vec<u8>) -> Self {
        Self {
            code: 0,
            key,
            value,
        }
    }

    pub fn error(code: u32, key: String) -> Self {
        Self {
            code,
            key,
            ..Default::default()
        }
    }
}

impl TryFrom<Response> for Bytes {
    type Error = prost::EncodeError;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        let mut buf = BytesMut::new();
        value.encode(&mut buf).map(|()| buf.freeze())
    }
}

impl TryFrom<BytesMut> for Response {
    type Error = prost::DecodeError;

    fn try_from(buf: BytesMut) -> Result<Self, Self::Error> {
        Message::decode(buf)
    }
}
