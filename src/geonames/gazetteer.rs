use crate::geonames::{download, Data};
use crate::models::Gazetteer;
use crate::Country;

const GEONAMES_GAZETTEER_URL_BASE: &str = "https://download.geonames.org/export/dump";

pub(crate) fn get_gazetteer_url(country: &Country) -> String {
    if [
        Country::GreatBritainFull,
        Country::UnitedKingdomFull,
        Country::NetherlandsFull,
        Country::CanadaFull,
    ]
    .contains(country)
    {
        panic!("Country {} not supported for gazetteer", country)
    }

    format!("{}/{}.zip", GEONAMES_GAZETTEER_URL_BASE, country)
}

pub fn load_gazetteer_data(data: &str) -> Vec<Gazetteer> {
    log::debug!("Parsing geonames data");
    let data: Vec<Gazetteer> = data
        .lines()
        .map(|line| {
            let fields: Vec<&str> = line.split('\t').collect();
            Gazetteer {
                id: fields[0].parse().unwrap(),
                name: fields[1].to_string(),
                asciiname: fields[2].to_string(),
                alternate_names: fields[3].split(',').map(|s| s.to_string()).collect(),
                latitude: fields[4].parse().unwrap(),
                longitude: fields[5].parse().unwrap(),
                feature_class: fields[6].to_string(),
                feature_code: fields[7].to_string(),
                country_code: fields[8].to_string(),
                cc2: fields[9].to_string(),
                admin1_code: fields.get(10).map(|s| s.to_string()),
                admin2_code: fields.get(11).map(|s| s.to_string()),
                admin3_code: fields.get(12).map(|s| s.to_string()),
                admin4_code: fields.get(13).map(|s| s.to_string()),
                population: fields[14].parse().unwrap_or_default(),
                elevation: fields[15].parse().unwrap_or_default(),
                dem: fields[16].parse().unwrap(),
                timezone: fields[17].to_string(),
                modification_date: chrono::NaiveDate::parse_from_str(fields[18], "%Y-%m-%d")
                    .unwrap(),
            }
        })
        .collect();

    log::debug!("Parsed {} records", data.len());

    data
}

/// Get Gazetteer data for a specific country.
///
/// # Arguments
///
/// * `country` - A `Country` enum representing the country.
///
/// # Returns
///
/// A `Vec` of `Gazetteer` structs.
pub fn get_gazetteer_data(country: Country) -> Vec<Gazetteer> {
    let data = download(&country, Data::Gazetteer).unwrap();
    load_gazetteer_data(&data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_load_gazzeteer() {
        let gazetteer_data = get_gazetteer_data(Country::UnitedKingdom);
        assert!(!gazetteer_data.is_empty());
    }
}
