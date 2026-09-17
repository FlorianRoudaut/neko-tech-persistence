use crate::{Key, Persisted};

#[derive(Debug)]
pub enum PersistenceError {
    NotFound,
    AlreadyExists,
    StorageError(String),
}

pub trait Repository<T: Persisted> {
    fn create(&mut self, item: T) -> Result<(), PersistenceError>;
    fn read_one_by_id(&self, key: &Key) -> Result<&T, PersistenceError>;
    fn read_one_by_name(&self, unique_name: &str) -> Result<&T, PersistenceError>;
    fn read_all(&self) -> Vec<&T>;
    fn update(&mut self, item: T) -> Result<(), PersistenceError>;
    fn delete(&mut self, key: &Key) -> Result<(), PersistenceError>;
}
