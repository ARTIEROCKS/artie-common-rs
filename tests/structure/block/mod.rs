use serde_json;

use artie_common::structure::Block;
use artie_common::structure::Field;
use artie_common::structure::Input;

#[test]
pub fn test_serialization(){
    let input_1 = Input::new("test", "test", vec![Field::new("field_a", "123.45")]);
    let input_2 = Input::new("test", "test", vec![Field::new("field_b", "abc")]);
    let mut block = Block::new("test", "test", "test", vec![input_1]);
    block.add_input(input_2);

    let serialized = serde_json::to_string(&block).unwrap();
    let deserialized: Block = serde_json::from_str(&serialized).unwrap();

    assert_eq!(block, deserialized);

}

#[test]
pub fn test_deserialization(){
    let input_1 = Input::new("test", "test", vec![Field::new("field_a", "123.45")]);
    let input_2 = Input::new("test", "test", vec![Field::new("field_b", "abc")]);
    let mut block = Block::new("test", "test", "test", vec![input_1]);
    block.add_input(input_2);

    let serialized = serde_json::to_string(&block).unwrap();
    let deserialized: Block = serde_json::from_str(&serialized).unwrap();

    assert_eq!(block, deserialized);
}

#[test]
pub fn test_add_input() {
    let input_1 = Input::new("test", "test", vec![Field::new("field_a", "123.45")]);
    let input_2 = Input::new("test", "test", vec![Field::new("field_b", "abc")]);
    let mut block = Block::new("test", "test", "test", vec![input_1]);
    block.add_input(input_2);

    assert_eq!(block.inputs.len(), 2);
}

#[test]
pub fn test_add_nested() {
    let input_1 = Input::new("name", "code", vec![Field::new("field_a", "123.45")]);
    let mut block = Block::new("test", "test", "test", vec![]);
    let nested_block = Block::new("test", "test", "test", vec![input_1]);
    block.add_nested(nested_block);
    assert_eq!(block.nested.len(), 1);
}

#[test]
pub fn test_block_eq() {
    let input_1 = Input::new("test", "test", vec![Field::new("field_a", "123.45")]);
    let input_2 = Input::new("test", "test", vec![Field::new("field_a", "123.45")]);
    let input_3 = Input::new("test", "test", vec![Field::new("field_b", "abc")]);

    let mut block_1 = Block::new("test", "test", "test", vec![input_1]);
    let block_2 = Block::new("test", "test", "test", vec![input_2]);
    assert_eq!(block_1, block_2);
    block_1.add_input(input_3);
    assert_ne!(block_1, block_2);
}
