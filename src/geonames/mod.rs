mod gazetteer;
mod postal;
use std::io::Read;

pub(crate) enum Data {
    Postal,
    Gazetteer,
}

pub fn get_temp_dir() -> String {
    let binding = temp_dir();
    let path = binding.to_str().unwrap();
    let path = path.strip_suffix(&get_os_separator());
    format!("{}{}geonames", path.unwrap(), get_os_separator())
}

pub fn get_os_separator() -> String {
    std::path::MAIN_SEPARATOR.to_string()
}

pub fn download(country: &Country, data_type: Data) -> Result<String, Box<dyn std::error::Error>> {
    let disable_cache = std::env::var("DISABLE_GEOCODER_CACHE").is_ok();
    let cache_dir = std::env::var("GEOCODER_CACHE_DIR").unwrap_or(get_temp_dir());

    log::debug!(
        "Cache dir: {} | Disable cache: {}",
        cache_dir,
        disable_cache
    );

    let url = match data_type {
        Data::Postal => postal::get_postal_url(country),
        Data::Gazetteer => gazetteer::get_gazetteer_url(country),
    };
    let cache_dir = match data_type {
        Data::Postal => format!("{}{}postal", cache_dir, get_os_separator()),
        Data::Gazetteer => format!("{}{}gazetteer", cache_dir, get_os_separator()),
    };
    let cache_path = format!("{}{}{}.txt", cache_dir, get_os_separator(), country);

    if !disable_cache && std::path::Path::new(&cache_path).exists() {
        log::debug!("Using cached data from {}", cache_path);
        let mut data = String::new();
        std::fs::File::open(cache_path)?.read_to_string(&mut data)?;
        return Ok(data);
    }

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
        std::fs::create_dir_all(cache_dir)?;
        std::fs::write(cache_path, &data)?;
    }

    Ok(data)
}

use crate::Country;
pub use gazetteer::get_gazetteer_data;
pub use postal::get_postal_data;
use std::env::temp_dir;

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_download_postal() {
        let data = download(&Country::UnitedStates, Data::Postal).unwrap();
        assert!(!data.is_empty());
    }

    #[test_log::test]
    fn test_download_gazetteer() {
        let data = download(&Country::UnitedStates, Data::Gazetteer).unwrap();
        assert!(!data.is_empty());
    }

    #[test_log::test]
    fn test_download_full_postal() {
        let data = download(&Country::GreatBritainFull, Data::Postal).unwrap();
        assert!(!data.is_empty());
    }
}
