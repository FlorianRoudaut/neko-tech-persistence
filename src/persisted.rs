use crate::{Key, PersistenceError};

pub trait Persisted: Sized {
    fn key(&self) -> &Key;
    fn persisted_type() -> &'static str;
    fn to_proto_bytes(items: &[Self]) -> Vec<u8>;
    fn from_proto_bytes(bytes: &[u8]) -> Result<Vec<Self>, PersistenceError>;
}
