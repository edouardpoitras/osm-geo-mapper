OSM Geo Mapper
==============

Navigate OpenStreetMap data in the terminal.
Will fetch OSM data using the address or latitude/longitude provided, convert to GeoJSON (requires dependencies), and display the resulting lines/points/polygons in the terminal.

You can optionally provide a GeoJSON file directly for viewing.

Allows for moving around the map:
 - Arrow keys for directional movement (also supports vi keys h,j,k,l)
 - 'z' to zoom in/out
 - Hold Shift to move 10x faster
 - 'q' or Esc to quit

## But Why

I had an idea for a terminal-based zombie survival game set wherever on earth the player chooses.

Turns out, just parsing and enumerating all types of OpenStreetMap data is a significant amount of work and worth it's own library.

Eventually this library will expose a read-only data structure used for querying OSM data by 2D coordinates roughly mimicking latitude and longitude degrees.

## Warning

This library isn't complete yet. There are OSM features/properties/details that I haven't implemented yet and results in unclassified/missing data. If you encounter such a case, please submit a issue with the address/lat/lon or geojsonfile attempted and I'll fix those issues as they arise.

Usage
=====

## CLI

    ./osm-geo-mapper --help

    Will fetch OpenStreetMap data, convert to GeoJSON, and display the resulting lines/points/polygons in the terminal.

    USAGE:
        osm-geo-mapper [FLAGS] [OPTIONS]

    FLAGS:
            --show-amenities     Display all amenities - can take a while and cover up overlapping features like buildings
            --show-boundaries    Display all boundaries - can take a while and cover up overlapping features like roads
        -h, --help               Prints help information
            --show-landuse       Display all landuse areas - can take a while and cover up overlapping features like buildings
            --show-leisure       Display all leisure areas - can cover up overlapping features like buildings
        -V, --version            Prints version information
    
    OPTIONS:
        -a, --address <address>              The address that will be used when fetching OpenStreetMap data
        -g, --geojson-file <geojson-file>    Optionally provide a geojson file directly to be parsed and displayed in the terminal
            --latitude <latitude>            The latitude that will be used when fetching OpenStreetMap data (ignored if address is provided)
            --longitude <longitude>          The longitude that will be used when fetching OpenStreetMap data (ignored if address is provided)
        -r, --radius <radius>                The radius of the area of land to retrieve in 100,000th of a lat/lon degree (roughly a meter) - defaults to 200 (0.002 degrees or ~200m). Significantly impacts loading times

    ./osm-geo-mapper --address "110 laurier avenue west ottawa ontario"

![OSM Geo Mapper](/osm-geo-mapper.png?raw=true)

## Library

See the tests/ folder for example usage, but it boils down to the following functions:

    pub fn address_to_mapper(address: String, radius: Option<u32>) -> Result<operations::Mapper, Box<dyn std::error::Error>>

`address_to_mapper` takes an address string and optionally a radius (in 100,000th of a degree, or roughly a meter) and returns a Mapper object.

    pub fn lat_lon_to_mapper(latitude: f64, longitude: f64, radius: Option<u32>) -> Result<operations::Mapper, Box<dyn std::error::Error>>

`lat_lon_to_mapper` does the same thing as above except it takes a latitude and longitude instead of an address.

    pub fn geojson_file_to_mapper(geojson_file: String, location: Option<operations::Location>) -> Result<operations::Mapper, Box<dyn std::error::Error>>

`geojson_file_to_mapper` takes a geojson file path directly and also returns a Mapper object. The `location` optional parameter is not useful yet.

The `Mapper` type is defined as follows:

    pub struct Mapper {
        pub data_structure: HashMap<gt::Coordinate<i32>, Rc<GeoTile>>;
        pub coordinates: gt::Coordinate<i32>
    }

`data_structure` is used to access the various GeoTiles by coordinates.
`coordinates` holds x/y coordinates of the address (if `address_to_mapper` was used) or to the lat/lon initially provided. They are no longer in the original lat/lon format but in the data structure's coordinate system (each step is 100,000th of a degree, or roughly one meter).

If you wanted to get the GeoTile at the real-world lat/lon of -75.690308/45.421106, you would get the tile in the data structure at geo_types::Coordinate { x: -7569031, y: 4542111 }). You can directly convert from lat/lon to x/y by multipling by 100,000 and converting to a signed 32-bit integer. You can also use the helper methods `osm_geo_mapper::operations::to_tile_scale(f64) -> i32` and `osm_geo_mapper::operations::from_tile_scale(i32) -> f64`.

A GeoTile can be many many things - see `features.rs`.

TODO
====

- Implement logic to choose a lat/lon in the middle of a geojson file if none is provided via command line
- Implement lazy-loading of more map data
- Implement (binary?) serialization of map data to be loaded/cached
- Add short-cut key info pane under details pane
- Continue adding missing properties/details/themes