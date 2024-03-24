mod geonames;
mod haversine;
mod models;

pub use models::{Accuracy, Country, GeoLocation, PostalData};

use crate::models::Gazetteer;
pub use geonames::{get_gazetteer_data, get_postal_data, invalidate_cache};
pub use haversine::calculate_distance;

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
        .iter().
        filter(|geoname| geoname.geolocation.is_some())
            .min_by_key(|geoname| {
                geoname.geolocation.clone().unwrap().distance(&location)  as i32
            }
        )
}



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
    geonames_data.iter()
        .filter(|geoname| geoname.geolocation.is_some())
        .min_by_key(|geoname| {
            geoname.geolocation.clone().unwrap().distance(&location) as i32
        })
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
    let mut postcodes: Vec<&str> = geonames_data
        .iter()
        .filter(|geoname| geoname.geolocation.is_some())
        .filter(|geoname| {
            geoname.geolocation.clone().unwrap().distance(&location) <= radius
        })
        .map(|geoname| geoname.postal_code.as_str())
        .collect();
    postcodes.dedup();

    postcodes
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
    let mut places: Vec<&str> = geonames_data
        .iter()
        .filter(|geoname| geoname.geolocation.is_some())
        .filter(|geoname| {
            geoname.geolocation.clone().unwrap().distance(&location) <= radius
        })
        .map(|geoname| geoname.name.as_str())
        .collect();
    places.dedup();

    places
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
    let mut loc: Vec<&PostalData> = geonames_data
        .iter()
        .filter(|geoname| geoname.geolocation.is_some())
        .filter(|geoname| {
            geoname.geolocation.clone().unwrap().distance(&location) <= radius
        })
        .collect();
    loc.dedup();

    loc
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_postal_data;

    static GEONAMES_POSTAL_DATA: once_cell::sync::Lazy<Vec<PostalData>> =
        once_cell::sync::Lazy::new(|| get_postal_data(Country::All));

    static GEONAMES_GAZETTEER_DATA: once_cell::sync::Lazy<Vec<Gazetteer>> =
        once_cell::sync::Lazy::new(|| {
            let geonames_data = crate::geonames::get_gazetteer_data(Country::GreatBritain);
            geonames_data
        });

    #[test_log::test]
    fn test_get_nearest_postcode() {
        let location = GeoLocation {
            latitude: 51.7923246977375,
            longitude: 0.629834723775309,
        };

        let geonames_data = GEONAMES_POSTAL_DATA.clone();

        let nearest_postcode = get_nearest_postcode(location, &geonames_data).unwrap();

        assert_eq!(nearest_postcode.postal_code, "CM8");
    }

    #[test_log::test]
    fn test_get_postcode_location() {
        let postcode = "CM8";
        let geonames_data = GEONAMES_POSTAL_DATA.clone();

        let location = get_postcode_location(postcode, &geonames_data).unwrap();

        assert!((location.latitude - 51.7923246977375).abs() < 0.1);
        assert!((location.longitude - 0.629834723775309).abs() < 0.1);
    }

    #[test_log::test]
    fn test_get_postcodes_within_radius() {
        let location = GeoLocation {
            latitude: 51.7923246977375,
            longitude: 0.629834723775309,
        };

        let radius = 10.0;

        let geonames_data = GEONAMES_POSTAL_DATA.clone();

        let postcodes = get_postcodes_within_radius(location, radius, &geonames_data);

        assert!(postcodes.len() > 1);
        let min_expected = ["CM3", "CM7", "CM8", "CM9", "CM98", "CO5", "CO6"];

        for postcode in min_expected.iter() {
            assert!(postcodes.contains(postcode));
        }
    }

    #[test_log::test]
    fn test_get_geonames_within_radius() {
        let location = GeoLocation {
            latitude: 51.7923246977375,
            longitude: 0.629834723775309,
        };

        let radius = 10.0;

        let geonames_data = GEONAMES_POSTAL_DATA.clone();

        let locations = get_postal_data_within_radius(location, radius, &geonames_data);

        assert!(locations.len() > 1);
        let min_expected = ["CM3", "CM7", "CM8", "CM9", "CM98", "CO5", "CO6"];

        for postcode in min_expected.iter() {
            assert!(locations
                .iter()
                .any(|geoname| geoname.postal_code == *postcode));
        }
    }

    #[test_log::test]
    fn test_get_nearest_place() {
        let location = GeoLocation {
            latitude: 51.7923246977375,
            longitude: 0.629834723775309,
        };

        let geonames_data = GEONAMES_GAZETTEER_DATA.clone();

        let nearest_place = get_nearest_place(location, &geonames_data).unwrap();

        assert_eq!(nearest_place.name, "Witham Blunts Hall");
    }

    #[test_log::test]
    fn test_get_place_location() {
        let place = "Chelmsford";
        let geonames_data = GEONAMES_GAZETTEER_DATA.clone();

        let location = get_place_location(place, &geonames_data).unwrap();

        assert!((location.latitude - 51.735586).abs() < 0.1);
        assert!((location.longitude - 0.468549).abs() < 0.1);
    }

    #[test_log::test]
    fn test_get_places_within_radius() {
        let location = GeoLocation {
            latitude: 51.7923246977375,
            longitude: 0.629834723775309,
        };

        let radius = 10.0;

        let geonames_data = GEONAMES_GAZETTEER_DATA.clone();

        let places = get_places_within_radius(location, radius, &geonames_data);

        assert!(places.len() > 1);
        let min_expected = [
            "Woodham Mortimer",
            "Witham",
            "Wickham Bishops",
            "White Notley",
        ];

        for place in min_expected.iter() {
            assert!(places.contains(place));
        }
    }

    #[test_log::test]
    fn test_invalidate_cache() {
        invalidate_cache();
    }
}
