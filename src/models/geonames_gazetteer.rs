use chrono::NaiveDate;

use crate::GeoLocation;

#[derive(Debug, Clone, PartialEq)]
pub struct Gazetteer {
    // ID of record in geonames db
    pub id: i64,
    // name of geographical point (utf8)
    pub name: String,
    // name of geographical point in plain ascii characters
    pub asciiname: String,
    // alternative names for the geographical point
    pub alternate_names: Vec<String>,
    // latitude and longitude in decimal degrees
    pub geolocation: Option<GeoLocation>,
    // see http://www.geonames.org/export/codes.html
    pub feature_class: String,
    // see http://www.geonames.org/export/codes.html
    pub feature_code: String,
    // ISO-3166 2-letter country code, 2 characters
    pub country_code: String,
    // alternate country codes, comma separated, ISO-3166 2-letter country code
    pub alternate_country_codes: Vec<String>,
    // fipscode (subject to change to iso code), see exceptions below, see file admin1Codes.txt for display names of this code
    pub admin1_code: Option<String>,
    // code for the second administrative division, a county in the US, see file admin2Codes.txt
    pub admin2_code: Option<String>,
    // code for third level administrative division
    pub admin3_code: Option<String>,
    // code for fourth level administrative division
    pub admin4_code: Option<String>,
    // population
    pub population: i64,
    // elevation in meters,
    pub elevation: i64,
    // digital elevation model, srtm3 or gtopo30, average elevation of 3''x3'' (ca 90mx90m) or 30''x30'' (ca 900mx900m) area in meters, integer. srtm processed by cgiar/ciat.
    pub dem: i64,
    // the timezone id
    pub timezone: String,
    // date of last modification
    pub modification_date: NaiveDate,
}
