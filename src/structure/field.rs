#[derive(Debug, PartialEq)]
pub struct Field {
    pub name: String,
    pub value: String,
}

impl Field {
    // This associated function serves as a constructor for Field.
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
        }
    }

    pub fn is_numeric(&self) -> bool {
        self.value.parse::<f64>().is_ok()
    }

    pub fn value_as_double(&self) -> f64 {
        match self.value.parse::<f64>() {
            Ok(num) => num,
            Err(_) => 0.0,
        }
    }
}
