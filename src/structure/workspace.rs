use serde::{Deserialize, Serialize};
use crate::structure::block::Block;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub blocks: Vec<Block>,
}

impl Workspace {
    pub fn new(id: &str, name: &str, blocks: Vec<Block>) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            blocks,
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}