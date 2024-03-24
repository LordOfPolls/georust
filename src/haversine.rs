use crate::GeoLocation;

pub const EARTH_RADIUS: f64 = 6371.0;

#[derive(Debug)]
pub struct BoundingBox {
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_lon: f64,
    pub max_lon: f64,
}

impl BoundingBox {
    /// Create a new `BoundingBox` struct.
    ///
    /// # Arguments
    ///
    /// * `centre` - A `GeoLocation` struct representing the center of the bounding box.
    /// * `threshold` - A `f64` representing the threshold distance in kilometers.
    ///
    /// # Returns
    ///
    /// A `BoundingBox` struct representing the bounding box.
    pub fn new(centre: &GeoLocation, threshold: f64) -> Self {
        let lat_diff = threshold / EARTH_RADIUS.to_radians();
        let lon_diff = threshold / (EARTH_RADIUS * centre.latitude.to_radians().cos()).to_radians();

        BoundingBox {
            min_lat: centre.latitude - lat_diff,
            max_lat: centre.latitude + lat_diff,
            min_lon: centre.longitude - lon_diff,
            max_lon: centre.longitude + lon_diff,
        }
    }
}

/// Calculate the haversine distance between two locations.
///
/// # Arguments
///
/// * `location_1` - A `Location` struct representing the first location.
/// * `location_2` - A `Location` struct representing the second location.
///
/// # Returns
///
/// A `f64` representing the distance between the two locations in kilometers.
pub fn calculate_distance(location_1: &GeoLocation, location_2: &GeoLocation) -> f64 {
    let d_lat = (location_2.latitude - location_1.latitude).to_radians();
    let d_lon = (location_2.longitude - location_1.longitude).to_radians();

    let a = (d_lat / 2.0).sin() * (d_lat / 2.0).sin()
        + location_1.latitude.to_radians().cos()
            * location_2.latitude.to_radians().cos()
            * (d_lon / 2.0).sin()
            * (d_lon / 2.0).sin();
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    EARTH_RADIUS * c
}

/// Check if a location is within a bounding box.
///
/// # Arguments
///
/// * `location` - A `Location` struct representing the location.
/// * `bounding_box` - A `BoundingBox` struct representing the bounding box.
///
/// # Returns
///
/// A `bool` indicating whether the location is within the bounding box.
pub fn is_within_bounding_box(location: &GeoLocation, bounding_box: &BoundingBox) -> bool {
    location.latitude >= bounding_box.min_lat
        && location.latitude <= bounding_box.max_lat
        && location.longitude >= bounding_box.min_lon
        && location.longitude <= bounding_box.max_lon
}

#[cfg(test)]
mod tests {
    use crate::GeoLocation;

    use super::*;

    #[test_log::test]
    fn test_calculate_distance() {
        let location_1 = GeoLocation {
            latitude: -25.13275,
            longitude: -47.50261,
        };
        let location_2 = GeoLocation {
            latitude: -30.04997,
            longitude: 140.03919,
        };

        let distance = calculate_distance(&location_1, &location_2);

        assert!((distance - 13826.0).abs() < 1.0);
    }
}
