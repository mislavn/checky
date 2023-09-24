use std::fmt;

#[derive(Clone)]
pub struct Leaf {
    name: String,
    description: String,
}

pub enum LeafType<T> {
    String(StringType),
    Number(NumberType<T>),
    List(ListType<T>),
}

pub struct StringType {
    pub pattern: Option<String>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

pub struct NumberType<T> {
    pub min: Option<T>,
    pub max: Option<T>,
}

pub struct ListType<T> {
    pub min: Option<T>,
    pub max: Option<T>,
}

impl fmt::Debug for Leaf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Leaf {}: {}", self.name, self.description)
    }
}
