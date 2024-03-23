mod haversine;
mod load_geonames;
mod models;

pub use models::{Accuracy, Country, GeoLocation, PostalData};

pub use haversine::calculate_distance;
pub use load_geonames::get_postal_data;

/// Get the nearest postcode to a location.
///
/// # Arguments
///
/// * `location` - A `Location` struct representing the location.
/// * `geonames_data` - A slice of `GeoNamesData` structs.
///
/// # Returns
///
/// An `Option` containing a reference to the nearest `GeoNamesData` struct.
pub fn get_nearest_postcode(
    location: GeoLocation,
    geonames_data: &[PostalData],
) -> Option<&PostalData> {
    geonames_data
        .iter()
        .filter(|geoname| geoname.latitude.is_some() && geoname.longitude.is_some())
        .min_by_key(|geoname| {
            let location_2 = GeoLocation {
                latitude: geoname.latitude.unwrap(),
                longitude: geoname.longitude.unwrap(),
            };
            haversine::calculate_distance(&location, &location_2) as i32
        })
}

/// Get the location of a postcode.
///
/// # Arguments
///
/// * `postcode` - A `&str` representing the postcode.
/// * `geonames_data` - A slice of `GeoNamesData` structs.
///
/// # Returns
///
/// An `Option` containing a `Location` struct.
pub fn get_postcode_location(
    postcode: &str,
    geonames_data: &[PostalData],
) -> Option<GeoLocation> {
    geonames_data
        .iter()
        .filter(|geoname| geoname.postal_code == postcode)
        .filter_map(|geoname| {
            if let (Some(latitude), Some(longitude)) = (geoname.latitude, geoname.longitude) {
                Some(GeoLocation {
                    latitude,
                    longitude,
                })
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
/// * `geonames_data` - A slice of `GeoNamesData` structs.
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
        .filter(|geoname| geoname.latitude.is_some() && geoname.longitude.is_some())
        .filter(|geoname| {
            let location_2 = GeoLocation {
                latitude: geoname.latitude.unwrap(),
                longitude: geoname.longitude.unwrap(),
            };
            haversine::calculate_distance(&location, &location_2) <= radius
        })
        .map(|geoname| geoname.postal_code.as_str())
        .collect();
    postcodes.dedup();

    postcodes
}

/// Get all `GeoNamesData` structs within a certain radius of a location.
///
/// # Arguments
///
/// * `location` - A `Location` struct representing the location.
/// * `radius` - A `f64` representing the radius in kilometers.
/// * `geonames_data` - A slice of `GeoNamesData` structs.
///
/// # Returns
///
/// A `Vec` of `&GeoNamesData` containing the postcodes.
pub fn get_geonames_within_radius(
    location: GeoLocation,
    radius: f64,
    geonames_data: &[PostalData],
) -> Vec<&PostalData> {
    let mut loc: Vec<&PostalData> = geonames_data
        .iter()
        .filter(|geoname| geoname.latitude.is_some() && geoname.longitude.is_some())
        .filter(|geoname| {
            let location_2 = GeoLocation {
                latitude: geoname.latitude.unwrap(),
                longitude: geoname.longitude.unwrap(),
            };
            haversine::calculate_distance(&location, &location_2) <= radius
        })
        .collect();
    loc.dedup();

    loc
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_geonames::get_postal_data;

    static GEONAMES_DATA: once_cell::sync::Lazy<Vec<PostalData>> =
        once_cell::sync::Lazy::new(|| get_postal_data(Country::All));

    #[test_log::test]
    fn test_haversine() {
        let location_1 = GeoLocation {
            latitude: -25.13275,
            longitude: -47.50261,
        };
        let location_2 = GeoLocation {
            latitude: -30.04997,
            longitude: 140.03919,
        };

        let distance = haversine::calculate_distance(&location_1, &location_2);

        assert!((distance - 13826.0).abs() < 1.0);
    }

    #[test_log::test]
    fn test_load_geonames() {
        let geonames_data = GEONAMES_DATA.clone();

        assert!(geonames_data.len() > 100);

        let mut no_lat_long = 0;

        for geoname in geonames_data.iter() {
            assert!(!geoname.country_code.is_empty());
            assert!(!geoname.postal_code.is_empty());

            if geoname.latitude.is_none() || geoname.longitude.is_none() {
                no_lat_long += 1;
            } else {
                assert!(geoname.latitude.unwrap() >= -90.0 && geoname.latitude.unwrap() <= 90.0);
                assert!(
                    geoname.longitude.unwrap() >= -180.0 && geoname.longitude.unwrap() <= 180.0
                );

                assert!(geoname.accuracy != models::Accuracy::NoLocation);
            }
        }

        assert!(no_lat_long < geonames_data.len() / 4);
    }

    #[test_log::test]
    fn test_get_nearest_postcode() {
        let location = GeoLocation {
            latitude: 51.7923246977375,
            longitude: 0.629834723775309,
        };

        let geonames_data = GEONAMES_DATA.clone();

        let nearest_postcode = get_nearest_postcode(location, &geonames_data).unwrap();

        assert_eq!(nearest_postcode.postal_code, "CM8");
    }

    #[test_log::test]
    fn test_get_postcode_location() {
        let postcode = "CM8";
        let geonames_data = GEONAMES_DATA.clone();

        let location = get_postcode_location(postcode, &geonames_data).unwrap();

        assert!((location.latitude - 51.7923246977375).abs() < 0.1);
        assert!((location.longitude - 0.629834723775309).abs() < 0.1);
    }

    #[test_log::test]
    fn test_get_specific_country() {
        let geonames_data = get_postal_data(Country::GreatBritain);

        assert!(geonames_data.len() > 100);

        for geoname in geonames_data.iter() {
            assert_eq!(geoname.country_code, "GB");
        }

        let geonames_data = get_postal_data(Country::UnitedStates);

        assert!(geonames_data.len() > 100);

        for geoname in geonames_data.iter() {
            assert_eq!(geoname.country_code, "US");
        }
    }

    #[test_log::test]
    fn test_get_postcodes_within_radius() {
        let location = GeoLocation {
            latitude: 51.7923246977375,
            longitude: 0.629834723775309,
        };

        let radius = 10.0;

        let geonames_data = GEONAMES_DATA.clone();

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

        let geonames_data = GEONAMES_DATA.clone();

        let locations = get_geonames_within_radius(location, radius, &geonames_data);

        assert!(locations.len() > 1);
        let min_expected = ["CM3", "CM7", "CM8", "CM9", "CM98", "CO5", "CO6"];

        for postcode in min_expected.iter() {
            assert!(locations
                .iter()
                .any(|geoname| geoname.postal_code == *postcode));
        }
    }

    #[test_log::test]
    fn test_get_full_geonames_data() {
        let geonames_data = get_postal_data(Country::GreatBritainFull);

        assert!(geonames_data.len() > 100);

        for geoname in geonames_data.iter() {
            assert!(!geoname.country_code.is_empty());
            assert!(!geoname.postal_code.is_empty());
        }
    }
}
