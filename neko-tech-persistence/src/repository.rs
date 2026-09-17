use crate::{Key, Persisted};

#[derive(Debug)]
pub enum PersistenceError {
    NotFound,
    AlreadyExists,
    StorageError(String),
}

pub trait Repository<T: Persisted> {
    fn create(&mut self, item: T) -> Result<(), PersistenceError>;
    fn read(&self, key: &Key) -> Result<&T, PersistenceError>;
    fn update(&mut self, item: T) -> Result<(), PersistenceError>;
    fn delete(&mut self, key: &Key) -> Result<(), PersistenceError>;
}
