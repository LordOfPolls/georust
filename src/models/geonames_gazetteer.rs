use chrono::NaiveDate;
use crate::GeoLocation;

#[derive(Debug, Clone, PartialEq)]
pub struct Gazetteer {
    pub id: i64,
    pub name: String,
    pub asciiname: String,
    pub alternate_names: Vec<String>,
    pub geolocation: Option<GeoLocation>,
    pub feature_class: String,
    pub feature_code: String,
    pub country_code: String,
    pub cc2: String,
    pub admin1_code: Option<String>,
    pub admin2_code: Option<String>,
    pub admin3_code: Option<String>,
    pub admin4_code: Option<String>,
    pub population: i64,
    pub elevation: i64,
    pub dem: i64,
    pub timezone: String,
    pub modification_date: NaiveDate,
}
