use crate::GeoLocation;

pub const EARTH_RADIUS: f64 = 6371.0;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GeoLocation;

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
