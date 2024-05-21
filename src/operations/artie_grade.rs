pub fn calculate_grade(maximum_distance: f64, current_distance: f64, maximum_grade: f64) -> f64 {
    
    // Gets the minimum value from maximumDistance and currentDistance
    let min_distance = maximum_distance.min(current_distance);

    // Formula that calculates the grade
    maximum_grade - ((maximum_grade*min_distance)/maximum_distance)
}