use artie_common::structure::Block;
use artie_common::structure::Field;

#[test]
pub fn test_add_field() {
    let field_1 = Field::new("test", "123.45");
    let field_2 = Field::new("test", "abc");
    let mut block = Block::new("test", "test", "test", vec![field_1]);
    block.add_field(field_2);
    assert_eq!(block.fields.len(), 2);
}

#[test]
pub fn test_add_nested() {
    let field_1 = Field::new("test", "123.45");
    let mut block = Block::new("test", "test", "test", vec![]);
    let nested_block = Block::new("test", "test", "test", vec![field_1]);
    block.add_nested(nested_block);
    assert_eq!(block.nested.len(), 1);
}

#[test]
pub fn test_block_eq() {
    let field_1 = Field::new("test", "123.45");
    let field_2 = Field::new("test", "123.45");
    let field_3 = Field::new("test", "abc");
    let mut block_1 = Block::new("test", "test", "test", vec![field_1]);
    let block_2 = Block::new("test", "test", "test", vec![field_2]);
    assert_eq!(block_1, block_2);
    block_1.add_field(field_3);
    assert_ne!(block_1, block_2);
}
