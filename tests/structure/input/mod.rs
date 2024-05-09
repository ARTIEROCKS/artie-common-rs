use artie_common::structure::Input;
use artie_common::structure::Field;

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
