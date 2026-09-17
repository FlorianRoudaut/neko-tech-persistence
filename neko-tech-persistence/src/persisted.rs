use crate::Key;

pub trait Persisted {
    fn key(&self) -> &Key;
    fn persisted_type(&self) -> &str;
}
