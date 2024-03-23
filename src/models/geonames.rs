#[derive(Debug, Clone, PartialEq)]
pub struct GeoNamesData {
    // iso country code, 2 characters
    pub country_code: String,
    // max 20 character postal code
    pub postal_code: String,
    // max 180 character place name
    pub place_name: Option<String>,
    // 1. order subdivision (state)
    pub admin_name1: Option<String>,
    // 1. order subdivision (state) Code
    pub admin_code1: Option<String>,
    // 2. order subdivision (county/province),
    pub admin_name2: Option<String>,
    // 2. order subdivision (county/province) Code
    pub admin_code2: Option<String>,
    // 3. order subdivision (community)
    pub admin_name3: Option<String>,
    // 3. order subdivision (community) Code
    pub admin_code3: Option<String>,
    // estimated latitude (wgs84)
    pub latitude: Option<f64>,
    // estimated longitude (wgs84)
    pub longitude: Option<f64>,
    // accuracy of lat/lng from 1=estimated, 4=geonameid, 6=centroid of addresses or shape
    pub accuracy: i32,
}
