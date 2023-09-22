use std::fmt;

#[derive(Clone)]
pub struct Leaf {
    name: String,
    description: String,
}

impl fmt::Debug for Leaf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Leaf {}: {}", self.name, self.description)
    }
}
