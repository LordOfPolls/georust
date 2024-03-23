use crate::{Country, GeoNamesData};
use std::env::temp_dir;
use std::io::Read;
const GEONAMES_URL: &str = "http://download.geonames.org/export/zip";

fn get_temp_dir() -> String {
    format!("{}/geonames", temp_dir().to_str().unwrap())
}

fn get_url(country: &Country) -> String {
    if vec![Country::GreatBritainFull, Country::UnitedKingdomFull, Country::NetherlandsFull, Country::CanadaFull].contains(country) {
        return format!("{}/{}.csv.zip", GEONAMES_URL, country)
    }

    return format!("{}/{}.zip", GEONAMES_URL, country)
}

pub fn download_data(country: &Country) -> Result<String, Box<dyn std::error::Error>> {
    let disable_cache = std::env::var("DISABLE_GEOCODER_CACHE").is_ok();
    let cache_dir = std::env::var("GEOCODER_CACHE_DIR").unwrap_or(get_temp_dir());

    log::debug!(
        "Cache dir: {} | Disable cache: {}",
        cache_dir,
        disable_cache
    );

    if !disable_cache {
        let cache_file = format!("{}/{}.txt", cache_dir, country);
        if std::path::Path::new(&cache_file).exists() {
            log::debug!("Using cached data from {}", cache_file);
            let mut data = String::new();
            std::fs::File::open(cache_file)?.read_to_string(&mut data)?;
            return Ok(data);
        }
    }

    let url = get_url(country);
    log::info!("Downloading data from {}", url);
    let response = reqwest::blocking::get(url)?;
    let zip_file = response.bytes()?;
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(zip_file))?;

    let file_name = format!("{}.txt", country);
    let mut data_file = archive.by_name(&file_name)?;
    let mut data = String::new();
    data_file.read_to_string(&mut data)?;

    if !disable_cache {
        log::debug!("Caching data to {}", cache_dir);
        let cache_file = format!("{}/{}.txt", cache_dir, country);
        std::fs::create_dir_all(cache_dir)?;
        std::fs::write(cache_file, &data)?;
    }

    Ok(data)
}

pub fn load_data(data: &str) -> Vec<GeoNamesData> {
    log::debug!("Parsing geonames data");
    let data: Vec<GeoNamesData> = data
        .lines()
        .map(|line| {
            let fields: Vec<&str> = line.split('\t').collect();
            GeoNamesData {
                country_code: fields[0].to_string(),
                postal_code: fields[1].to_string(),
                place_name: fields.get(2).map(|s| s.to_string()),
                admin_name1: fields.get(3).map(|s| s.to_string()),
                admin_code1: fields.get(4).map(|s| s.to_string()),
                admin_name2: fields.get(5).map(|s| s.to_string()),
                admin_code2: fields.get(6).map(|s| s.to_string()),
                admin_name3: fields.get(7).map(|s| s.to_string()),
                admin_code3: fields.get(8).map(|s| s.to_string()),
                latitude: fields[9].parse().ok(),
                longitude: fields[10].parse().ok(),
                accuracy: fields[11].parse().unwrap(),
            }
        })
        .collect();

    log::debug!("Parsed {} geonames entries", data.len());

    data
}

/// Get geonames data for a specific country.
///
/// # Arguments
///
/// * `country` - A `Country` enum representing the country.
///
/// # Returns
///
/// A `Vec` of `GeoNamesData` structs.
pub fn get_geonames_data(country: Country) -> Vec<GeoNamesData> {
    let data = download_data(&country).unwrap();
    load_data(&data)
}
