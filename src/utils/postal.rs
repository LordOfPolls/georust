use crate::{haversine, BoundingBox, GeoLocation, PostalData};

/// Get the nearest postcode to a location.
///
/// # Arguments
///
/// * `location` - A `Location` struct representing the location.
/// * `geonames_data` - A slice of `PostalData` structs.
///
/// # Returns
///
/// An `Option` containing a reference to the nearest `PostalData` struct.
pub fn get_nearest_postcode(
    location: GeoLocation,
    geonames_data: &[PostalData],
) -> Option<&PostalData> {
    geonames_data
        .iter()
        .filter(|geoname| geoname.geolocation.is_some())
        .min_by_key(|geoname| geoname.geolocation.clone().unwrap().distance(&location) as i32)
}

/// Get the nearest postcode to a location with a bounding box.
/// Used to filter out postcodes that are too far away.
///
/// # Arguments
///
/// * `location` - A `Location` struct representing the location.
/// * `geonames_data` - A slice of `PostalData` structs.
/// * `threshold` - A `f64` representing the maximum distance in kilometers.
///
/// # Returns
///
/// An `Option` containing a reference to the nearest `PostalData` struct.
pub fn get_nearest_postcode_with_bounding(
    location: GeoLocation,
    geonames_data: &[PostalData],
    threshold: f64,
) -> Option<&PostalData> {
    let bounds: BoundingBox = BoundingBox::new(&location, threshold);

    geonames_data
        .iter()
        .filter(|geoname| geoname.geolocation.is_some())
        .filter(|geoname| {
            haversine::is_within_bounding_box(&geoname.geolocation.clone().unwrap(), &bounds)
        })
        .min_by_key(|geoname| geoname.geolocation.clone().unwrap().distance(&location) as i32)
}

/// Get the location of a postcode.
///
/// # Arguments
///
/// * `postcode` - A `&str` representing the postcode.
/// * `geonames_data` - A slice of `PostalData` structs.
///
/// # Returns
///
/// An `Option` containing a `Location` struct.
pub fn get_postcode_location(postcode: &str, geonames_data: &[PostalData]) -> Option<GeoLocation> {
    geonames_data
        .iter()
        .filter(|geoname| geoname.postal_code == postcode)
        .filter_map(|geoname| {
            if geoname.geolocation.is_some() {
                Some(geoname.geolocation.clone().unwrap())
            } else {
                None
            }
        })
        .next()
}

/// Get all postcodes within a certain radius of a location.
///
/// # Arguments
///
/// * `location` - A `Location` struct representing the location.
/// * `radius` - A `f64` representing the radius in kilometers.
/// * `geonames_data` - A slice of `PostalData` structs.
///
/// # Returns
///
/// A `Vec` of `&str` containing the postcodes.
pub fn get_postcodes_within_radius(
    location: GeoLocation,
    radius: f64,
    geonames_data: &[PostalData],
) -> Vec<&str> {
    let bounds: BoundingBox = BoundingBox::new(&location, radius);

    let postcodes: Vec<&str> = geonames_data
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
        .map(|geoname| geoname.postal_code.as_str())
        .collect();

    postcodes
}

/// Get all `PostalData` structs within a certain radius of a location.
///
/// # Arguments
///
/// * `location` - A `Location` struct representing the location.
/// * `radius` - A `f64` representing the radius in kilometers.
/// * `geonames_data` - A slice of `PostalData` structs.
///
/// # Returns
///
/// A `Vec` of `&PostalData` containing the postcodes.
pub fn get_postal_data_within_radius(
    location: GeoLocation,
    radius: f64,
    geonames_data: &[PostalData],
) -> Vec<&PostalData> {
    let bounds: BoundingBox = BoundingBox::new(&location, radius);

    let mut loc: Vec<&PostalData> = geonames_data
        .iter()
        .filter(|geoname| geoname.geolocation.is_some())
        .filter(|geoname| {
            haversine::is_within_bounding_box(&geoname.geolocation.clone().unwrap(), &bounds)
        })
        .filter(|geoname| geoname.geolocation.clone().unwrap().distance(&location) <= radius)
        .collect();
    loc.dedup();

    loc
}

/// Get postcode data for a given postcode.
///
/// # Arguments
/// * `postcode` - A `&str` representing the postcode.
/// * `geonames_data` - A slice of `PostalData` structs.
///
/// # Returns
///
/// An `Option` containing a `PostalData` struct.
pub fn get_postcode(
    postcode: &str,
    geonames_data: &[PostalData],
) -> Option<PostalData> {
    geonames_data
        .iter()
        .filter(|geoname| geoname.postal_code == postcode)
        .cloned()
        .next()
}

