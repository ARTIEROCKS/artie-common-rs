use crate::structure::input::Input;

#[derive(Debug, PartialEq)]
pub struct Block {
    pub id: String,
    pub name: String,
    pub family: String,
    pub inputs: Vec<Input>,
    pub next: Option<Box<Block>>,
    pub previous: Option<Box<Block>>,
    pub parent: Option<Box<Block>>,
    pub nested: Vec<Block>,
}

impl Block {
    pub fn new(id: &str, name: &str, family: &str, inputs: Vec<Input>) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            family: family.to_string(),
            inputs,
            next: None,
            previous: None,
            parent: None,
            nested: Vec::new(),
        }
    }

    pub fn add_input(&mut self, input: Input) {
        self.inputs.push(input);
    }

    pub fn add_nested(&mut self, block: Block) {
        self.nested.push(block);
    }
}