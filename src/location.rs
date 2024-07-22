mod location;

#[derive(Debug)]
pub struct Location {
    name: String,
    coordinate: Coordinate
}

// Note that to return you shouldn't have the semicolon
pub impl Location {
    fn description(&self) -> String {
        format!("Location {} has coordinates of {}", self.name, self.coordinate.description())
    }

    fn new(name: String, latitude: f64, longitude: f64) -> Self {
        Self {
            name, 
            coordinate: Coordinate::new(latitude, longitude)
        }
    }
}

#[derive(Debug)]
pub struct Coordinate {
    latitude: f64,
    longitude: f64
}

// Implement functionality for Coordinate 
// - here a convenience initializer(?) is that the right term
impl Coordinate {
    fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude, longitude
        }
    }

    // simple debug description - note that the &self is "borrowing" itself to avoid ownership
    // ? - do they get retain cycles in rust? hmmm - probably not since the compiler is so strict
    fn description(&self) -> String {
        format!("Latitude: {}, Longitude: {}", self.latitude, self.longitude)
    }
}