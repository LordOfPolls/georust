# GeoRust

A rust library for geocoding and reverse geocoding using the GeoNames dataset. 

---------


## Usage

```rust
use geocoder::{GeoLocation, Country, get_geonames_data, get_nearest_postcode};

fn main() {
    let geonames_data = get_geonames_data(Country::All);

    let location = GeoLocation {
        latitude: 51.7923246977375,
        longitude: 0.629834723775309,
    };

    let nearest_postcode = get_nearest_postcode(location, &geonames_data).unwrap();
    println!("Nearest postcode: {}", nearest_postcode.postal_code);
}
```

## Features

- Calculate the haversine distance between two locations
- Get the nearest postcode to a location
- Get the location of a postcode
- Get all postcodes within a certain radius of a location


## Configuration

The library uses a cache directory to store downloaded GeoNames data. 
By default, it uses a directory named geonames in the system's temporary directory. 
You can override this by setting the `GEOCODER_CACHE_DIR` environment variable.

To disable caching entirely, set the `DISABLE_GEOCODER_CACHE` environment variable to any value.

Please note, you are encouraged to _"be a good neighbour"_ and use caching to reduce the load on the GeoNames servers.


## Contributing

Contributions are welcome! Please open an issue or submit a pull request.