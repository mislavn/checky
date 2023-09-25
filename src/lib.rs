use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Serialize, Deserialize)]
pub struct Leaf {
    name: String,
    description: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum LeafType<T> {
    String(StringType),
    Number(NumberType<T>),
    List(ListType<T>),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct StringType {
    pub pattern: Option<String>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NumberType<T> {
    pub min: Option<T>,
    pub max: Option<T>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ListType<T> {
    pub min: Option<T>,
    pub max: Option<T>,
}

impl fmt::Debug for Leaf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Leaf {}: {}", self.name, self.description)
    }
}
