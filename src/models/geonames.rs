use std::fmt::Display;
use std::str::FromStr;

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
    // accuracy of lat/lng
    pub accuracy: Accuracy,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Accuracy {
    NoLocation,
    NoAccuracyData,
    Estimated,
    SamePostalCodeOtherName,
    GeonameId,
    Centroid,
}

impl FromStr for Accuracy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // higher is better
            "0" => Ok(Accuracy::NoLocation),
            "1" => Ok(Accuracy::Estimated),
            "3" => Ok(Accuracy::SamePostalCodeOtherName),
            "4" => Ok(Accuracy::GeonameId),
            "6" => Ok(Accuracy::Centroid),
            _ => Ok(Accuracy::NoAccuracyData)
        }
    }
}

impl Display for Accuracy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Accuracy::Estimated => write!(f, "Estimated as average from numerically neighbouring postal codes"),
            Accuracy::SamePostalCodeOtherName => write!(f, "Same postal code, other name"),
            Accuracy::GeonameId => write!(f, "Place name from geoname id"),
            Accuracy::Centroid => write!(f, "Postal code area centroid"),
            Accuracy::NoAccuracyData => write!(f, "No accuracy data"),
            Accuracy::NoLocation => write!(f, "No location"),
        }
    }
}