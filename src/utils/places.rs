use crate::{haversine, BoundingBox, Gazetteer, GeoLocation};

/// Get the nearest place to a location.
///
/// # Arguments
///
/// * `location` - A `Location` struct representing the location.
/// * `geonames_data` - A slice of `Gazetteer` structs.
///
/// # Returns
///
/// An `Option` containing a reference to the nearest `Gazetteer` struct.
pub fn get_nearest_place(location: GeoLocation, geonames_data: &[Gazetteer]) -> Option<&Gazetteer> {
    geonames_data
        .iter()
        .filter(|geoname| geoname.geolocation.is_some())
        .min_by_key(|geoname| geoname.geolocation.clone().unwrap().distance(&location) as i32)
}

/// Get the nearest place to a location with a bounding box.
/// Used to filter out places that are too far away.
///
/// # Arguments
///
/// * `location` - A `Location` struct representing the location.
/// * `geonames_data` - A slice of `Gazetteer` structs.
/// * `threshold` - A `f64` representing the maximum distance in kilometers.
///
/// # Returns
///
/// An `Option` containing a reference to the nearest `Gazetteer` struct.
pub fn get_nearest_place_with_bounding(
    location: GeoLocation,
    geonames_data: &[Gazetteer],
    threshold: f64,
) -> Option<&Gazetteer> {
    let bounds: BoundingBox = BoundingBox::new(&location, threshold);

    geonames_data
        .iter()
        .filter(|geoname| geoname.geolocation.is_some())
        .filter(|geoname| {
            haversine::is_within_bounding_box(&geoname.geolocation.clone().unwrap(), &bounds)
        })
        .min_by_key(|geoname| geoname.geolocation.clone().unwrap().distance(&location) as i32)
}

/// Get the location of a place.
///
/// # Arguments
///
/// * `place` - A `&str` representing the place.
/// * `geonames_data` - A slice of `Gazetteer` structs.
///
/// # Returns
///
/// An `Option` containing a `Location` struct.
pub fn get_place_location(place: &str, geonames_data: &[Gazetteer]) -> Option<GeoLocation> {
    geonames_data
        .iter()
        .filter(|geoname| {
            geoname.name == place
                || geoname.asciiname == place
                || geoname.alternate_names.contains(&place.to_string())
        })
        .filter_map(|geoname| {
            if geoname.geolocation.is_some() {
                Some(geoname.geolocation.clone().unwrap())
            } else {
                None
            }
        })
        .next()
}

/// Get all places within a certain radius of a location.
///
/// # Arguments
///
/// * `location` - A `Location` struct representing the location.
/// * `radius` - A `f64` representing the radius in kilometers.
/// * `geonames_data` - A slice of `Gazetteer` structs.
///
/// # Returns
///
/// A `Vec` of `&str` containing the places.
pub fn get_places_within_radius(
    location: GeoLocation,
    radius: f64,
    geonames_data: &[Gazetteer],
) -> Vec<&str> {
    let bounds: BoundingBox = BoundingBox::new(&location, radius);

    let places: Vec<&str> = geonames_data
        .iter()
        .filter(|geoname| geoname.geolocation.is_some())
        .filter(|geoname| {
            // bounding box can overshoot, so we use it as a first pass
            haversine::is_within_bounding_box(&geoname.geolocation.clone().unwrap(), &bounds)
        })
        .filter(|geoname| {
            // then we filter out the ones that are still too far away
            geoname.geolocation.clone().unwrap().distance(&location) <= radius
        })
        .map(|geoname| geoname.name.as_str())
        .collect();

    places
}
