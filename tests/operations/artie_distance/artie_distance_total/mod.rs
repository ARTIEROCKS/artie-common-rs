use artie_common::structure::ArtieDistance;
use artie_common::operations::artie_distance::calculate_total_distance;

#[test]
pub fn test_artie_distance_total() {
    let mut artie_distance = ArtieDistance::new();
    artie_distance.family_distance = 1.0;
    artie_distance.block_distance = 2.0;
    artie_distance.position_distance = 3.0;
    artie_distance.input_distance = 4.0;

    calculate_total_distance(&mut artie_distance);

    let tolerance = 1e-10;
    assert!((artie_distance.total_distance - 3.25).abs() < tolerance, "artie_distance.total_distance = {}", artie_distance.total_distance);
}