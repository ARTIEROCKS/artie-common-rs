use crate::structure::field::Field;

#[derive(Debug, PartialEq)]
pub struct Block {
    pub id: String,
    pub name: String,
    pub family: String,
    pub fields: Vec<Field>,
    pub next: Option<Box<Block>>,
    pub previous: Option<Box<Block>>,
    pub parent: Option<Box<Block>>,
    pub nested: Vec<Block>,
}

impl Block {
    pub fn new(id: &str, name: &str, family: &str, fields: Vec<Field>) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            family: family.to_string(),
            fields,
            next: None,
            previous: None,
            parent: None,
            nested: Vec::new(),
        }
    }

    pub fn add_field(&mut self, field: Field) {
        self.fields.push(field);
    }

    pub fn add_nested(&mut self, block: Block) {
        self.nested.push(block);
    }

    pub fn is_numeric(&self) -> bool {
        self.fields.iter().all(|field| field.is_numeric())
    }
}