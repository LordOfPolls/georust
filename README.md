# GeoRust

A rust library for geocoding and reverse geocoding using the GeoNames dataset.

![GitHub Release](https://img.shields.io/github/v/release/LordOfPolls/georust?label=GitHub%20Release)
![Crates.io Version](https://img.shields.io/crates/v/geo_rust?label=crates.io%20Version)
![Crates.io Total Downloads](https://img.shields.io/crates/d/geo_rust)
![Crates.io License](https://img.shields.io/crates/l/geo_rust)
---------


## Usage

Run 
```bash
cargo add geo_rust
```

Then you can use the library like this:

```rust
use geo_rust::{GeoLocation, Country, get_postal_data, get_nearest_postcode};

fn main() {
    let geonames_data = get_postal_data(Country::All);

    let location = GeoLocation {
        latitude: 51.7923246977375,
        longitude: 0.629834723775309,
    };

    let nearest_postcode = get_nearest_postcode(location, &geonames_data).unwrap();
    println!("Nearest postcode: {}", nearest_postcode.postal_code);
}
```

```rust
use geo_rust::{GeoLocation, Country, get_gazetteer_data, get_nearest_place};

fn main() {
    let geonames_data = get_gazetteer_data(Country::GreatBritain);

    let location = GeoLocation {
        latitude: 51.7923246977375,
        longitude: 0.629834723775309,
    };

    let nearest_place = get_nearest_place(location, &geonames_data).unwrap();
    println!("Nearest place: {}", nearest_place.name);
}
```

Documentation is available at [docs.rs](https://docs.rs/geo_rust/latest/geo_rust/)

## Features

* Calculate the haversine distance between two locations
* Get the nearest postcode to a location
* Get the location of a postcode
* Get all postcodes within a certain radius of a location
* Get the nearest place to a location
* Get the location of a place
* Get all places within a certain radius of a location
* Get all PostalData structs within a certain radius of a location


## Configuration

The library uses a cache directory to store downloaded GeoNames data. 
By default, it uses a directory named geonames in the system's temporary directory. 
You can override this by setting the `GEOCODER_CACHE_DIR` environment variable.

To disable caching entirely, set the `DISABLE_GEOCODER_CACHE` environment variable to any value.

Please note, you are encouraged to _"be a good neighbour"_ and use caching to reduce the load on the GeoNames servers.


## Contributing

Contributions are welcome! Please open an issue or submit a pull request.