pub use geonames::{get_gazetteer_data, get_postal_data, invalidate_cache};
pub use haversine::{calculate_distance, BoundingBox};
pub use models::{Accuracy, Country, Gazetteer, GeoLocation, PostalData};
pub use utils::*;

mod geonames;
mod haversine;
mod models;
mod utils;

#[cfg(test)]
mod tests {
    use crate::get_postal_data;

    use super::*;

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
    fn test_get_nearest_postcode_with_bounding() {
        let location = GeoLocation {
            latitude: 51.7923246977375,
            longitude: 0.629834723775309,
        };

        let geonames_data = GEONAMES_POSTAL_DATA.clone();

        let nearest_postcode =
            get_nearest_postcode_with_bounding(location, &geonames_data, 1.0).unwrap();

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
    fn test_get_nearest_place_with_bounding() {
        let location = GeoLocation {
            latitude: 51.7923246977375,
            longitude: 0.629834723775309,
        };

        let geonames_data = GEONAMES_GAZETTEER_DATA.clone();

        let nearest_place = get_nearest_place_with_bounding(location, &geonames_data, 1.0).unwrap();

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

        let radius = 100.0;

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
    fn test_get_postal_data() {
        let geonames_data = GEONAMES_POSTAL_DATA.clone();
        assert!(!geonames_data.is_empty());

        let postcode = get_postcode("SW1A", &geonames_data);

        assert!(postcode.is_some());
    }

    #[test_log::test]
    fn test_invalidate_cache() {
        invalidate_cache();
    }
}
