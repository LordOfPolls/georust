use crate::geonames::{download, Data};
use crate::{Country, PostalData, GeoLocation};

const GENONAMES_POSTAL_URL_BASE: &str = "http://download.geonames.org/export/zip";

pub(crate) fn get_postal_url(country: &Country) -> String {
    if [
        Country::GreatBritainFull,
        Country::UnitedKingdomFull,
        Country::NetherlandsFull,
        Country::CanadaFull,
    ]
    .contains(country)
    {
        return format!("{}/{}.csv.zip", GENONAMES_POSTAL_URL_BASE, country);
    }

    format!("{}/{}.zip", GENONAMES_POSTAL_URL_BASE, country)
}

pub fn load_postal_data(data: &str) -> Vec<PostalData> {
    log::debug!("Parsing geonames data");
    let data: Vec<PostalData> = data
        .lines()
        .map(|line| {
            let fields: Vec<&str> = line.split('\t').collect();
            PostalData {
                country_code: fields[0].to_string(),
                postal_code: fields[1].to_string(),
                place_name: fields.get(2).map(|s| s.to_string()),
                admin_name1: fields.get(3).map(|s| s.to_string()),
                admin_code1: fields.get(4).map(|s| s.to_string()),
                admin_name2: fields.get(5).map(|s| s.to_string()),
                admin_code2: fields.get(6).map(|s| s.to_string()),
                admin_name3: fields.get(7).map(|s| s.to_string()),
                admin_code3: fields.get(8).map(|s| s.to_string()),
                geolocation: Some(GeoLocation {
                    latitude: fields[9].parse().unwrap(),
                    longitude: fields[10].parse().unwrap(),
                }),

                accuracy: fields[11].parse().unwrap(),
            }
        })
        .collect();

    log::debug!("Parsed {} geonames entries", data.len());

    data
}

/// Get Postal data for a specific country.
///
/// # Arguments
///
/// * `country` - A `Country` enum representing the country.
///
/// # Returns
///
/// A `Vec` of `PostalData` structs.
pub fn get_postal_data(country: Country) -> Vec<PostalData> {
    let data = download(&country, Data::Postal).unwrap();
    load_postal_data(&data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_load_postal() {
        let data = get_postal_data(Country::UnitedKingdom);
        assert!(!data.is_empty());
    }
}
