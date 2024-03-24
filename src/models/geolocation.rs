use crate::haversine::calculate_distance;

#[derive(Debug, PartialEq, Clone)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
}


impl GeoLocation {
    pub fn distance(&self, other: &GeoLocation) -> f64 {
        calculate_distance(self, other)
    }
}