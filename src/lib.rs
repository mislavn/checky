use std::fmt;

#[derive(Clone)]
pub struct Leaf {
    name: String,
    description: String,
}

pub enum LeafType {
    String(StringType),
    Number(NumberType),
}

pub struct StringType {
    pub pattern: Option<String>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

pub struct NumberType {
    pub min: Option<usize>,
    pub max: Option<usize>,
}

impl fmt::Debug for Leaf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Leaf {}: {}", self.name, self.description)
    }
}
