use artie_common::operations::artie_grade::calculate_grade;

#[test]
pub fn test_calculate_grade() {
    // Test case 1
    let maximum_distance = 100.0;
    let current_distance = 50.0;
    let maximum_grade = 10.0;
    let expected = 5.0;
    let result = calculate_grade(maximum_distance, current_distance, maximum_grade);
    assert_eq!(expected, result);
    // Test case 2
    let maximum_distance = 100.0;
    let current_distance = 100.0;
    let maximum_grade = 10.0;
    let expected = 0.0;
    let result = calculate_grade(maximum_distance, current_distance, maximum_grade);
    assert_eq!(expected, result);
    // Test case 3
    let maximum_distance = 100.0;
    let current_distance = 0.0;
    let maximum_grade = 10.0;
    let expected = 10.0;
    let result = calculate_grade(maximum_distance, current_distance, maximum_grade);
    assert_eq!(expected, result);
}