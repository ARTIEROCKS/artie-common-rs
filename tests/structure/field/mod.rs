use artie_common::structure::Field;

#[test]
fn test_is_numeric() {

    let field_1 = Field::new("test", "123.45");
    let field_2 = Field::new("test", "abc");

    assert!(field_1.is_numeric());
    assert!(!field_2.is_numeric());
}

#[test]
fn test_value_as_double() {

    let field_1 = Field::new("test", "123.45");
    let field_2 = Field::new("test", "abc");

    assert_eq!(field_1.value_as_double(), 123.45);
    assert_eq!(field_2.value_as_double(), 0.0);
}

#[test]
fn test_field_eq() {

    let field_1 = Field::new("test", "123.45");
    let field_2 = Field::new("test", "123.45");
    let field_3 = Field::new("test", "abc");

    assert_eq!(field_1, field_2);
    assert_ne!(field_1, field_3);
}
