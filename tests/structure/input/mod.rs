use artie_common::structure::Input;
use artie_common::structure::Field;

#[test]
fn test_serialization(){
    let field_1 = Field::new("test", "123.45");
    let field_2 = Field::new("test", "abc");

    let serialized_1 = serde_json::to_string(&field_1).unwrap();
    let serialized_2 = serde_json::to_string(&field_2).unwrap();

    assert_eq!(serialized_1, r#"{"name":"test","value":"123.45"}"#);
    assert_eq!(serialized_2, r#"{"name":"test","value":"abc"}"#);
}

#[test]
fn test_deserialization(){
    let field_1 = Field::new("test", "123.45");
    let field_2 = Field::new("test", "abc");

    let serialized_1 = serde_json::to_string(&field_1).unwrap();
    let serialized_2 = serde_json::to_string(&field_2).unwrap();

    let deserialized_1: Field = serde_json::from_str(&serialized_1).unwrap();
    let deserialized_2: Field = serde_json::from_str(&serialized_2).unwrap();

    assert_eq!(field_1, deserialized_1);
    assert_eq!(field_2, deserialized_2);
}

#[test]
fn test_input_eq() {
    let field_1 = Field::new("test", "123.45");
    let field_2 = Field::new("test", "123.45");
    let field_3 = Field::new("test", "abc");
    let input_1 = Input::new("test", "123.45", vec![field_1]);
    let input_2 = Input::new("test", "123.45", vec![field_2]);
    let input_3 = Input::new("test", "abc", vec![field_3]);
    assert_eq!(input_1, input_2);
    assert_ne!(input_1, input_3);
} 
