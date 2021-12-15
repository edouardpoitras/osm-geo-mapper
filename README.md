OSM Geo Mapper
==============

Rust library for querying OpenStreetMap data by coordinates and a terminal application to browse OpenStreeMap data in the terminal.
Will parse OSM/PBF/GeoJSON files or fetch OSM data using an address or lat/lon, and display the resulting lines/points/polygons in the terminal.

Allows for moving around the map:
 - Arrow keys for directional movement (also supports vi keys h,j,k,l)
 - 'z' to zoom in/out
 - Hold 'Shift' to move 10x faster
 - 'Enter' to load more data at your current location (uses previously chosen radius)
 - 'q' or Esc to quit

## But Why

I had an idea for a terminal-based zombie survival game set wherever on earth the player chooses.

Turns out, just parsing and enumerating all types of OpenStreetMap data is a significant amount of work and worth it's own library.

This library exposes a thread-safe data structure used for querying OSM data by 2D coordinates roughly mimicking latitude and longitude degrees.

Usage
=====

## CLI

    ./osm-geo-mapper --help

    osm-geo-mapper 0.8.0

    Will display OpenStreeMap data (PBF/OSM/GeoJSON) lines/points/polygons in the terminal.

    USAGE:
        osm-geo-mapper [OPTIONS]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
    -a, --address <address>              The address that will be used when fetching OpenStreetMap data (ignored if OSM/PBF/GeoJSON file is provided)
    -g, --geojson-file <geojson-file>    Optionally provide a GeoJSON file directly to be parsed and displayed in the terminal
    --latitude <latitude>            The latitude that will be used when fetching OpenStreetMap data (ignored if address or OSM/PBF/GeoJSON file is provided)
    --longitude <longitude>          The longitude that will be used when fetching OpenStreetMap data (ignored if address or OSM/PBF/GeoJSON file is provided)
    -o, --osm-file <osm-file>            Optionally provide a OSM file directly to be parsed and displayed in the terminal
    -p, --pbf-file <pbf-file>            Optionally provide a PBF file directly to be parsed and displayed in the terminal
    -r, --radius <radius>                The radius of the area of land to retrieve in 100,000th of a lat/lon degree (roughly a meter at the equator) - defaults to 200 (0.002 degrees or ~200m). Significantly impacts loading times. Ignored if PBF/GeoJSON file is provided

    ./osm-geo-mapper --address "ottawa canada"

![OSM Geo Mapper](/ottawa.png?raw=true)

    ./osm-geo-mapper --address "vancouver canada"

![OSM Geo Mapper](/vancouver.png?raw=true)

## Library

See the tests/ folder for example usage, but it boils down to the following OSMGeoMapper methods:

    use osm_geo_mapper::interface::{ OSMGeoMapper, Location };

    OSMGeoMapper::from_address(address: String, radius: Option<u32>) -> Result<OSMGeoMapper, Box<dyn std::error::Error>>

`OSMGeoMapper::from_address` takes an address string and optionally a radius (in 100,000th of a degree, or roughly a meter at the equator) and returns an OSMGeoMapper object.

    OSMGeoMapper::from_lat_lon(latitude: f64, longitude: f64, radius: Option<u32>) -> Result<OSMGeoMapper, Box<dyn std::error::Error>>

`OSMGeoMapper::from_lat_lon` does the same thing as above except it takes a latitude and longitude instead of an address.

    OSMGeoMapper::from_geojson_file(geojson_file: String, location: Option<Location>) -> Result<OSMGeoMapper, Box<dyn std::error::Error>>

`OSMGeoMapper::from_geojson_file` takes a geojson file path directly and also returns a OSMGeoMapper object. The `location` optional parameter is not useful yet.

The `OSMGeoMapper` type is defined as follows:

    pub struct OSMGeoMapper {
        pub data_structure: Arc<RwLock<HashMap<geo_types::Coordinate<i32>, Vec<Arc<GeoTile>>>>>,
        pub coordinates: geo_types::Coordinate<i32>,
        pub radius: u32
    }

`data_structure` is used to access the various GeoTiles by coordinates. This data structure is thread-safe due to the `Arc<RwLock<>>` wrapper. Use `OSMGeoMapper::atomic_clone(&self)` or `OSMGeoMapper.data_structure.clone()` directly when sending it to another thread. Use `OSMGeoMapper::get/get_real()` or `data_structure.read()/try_read()` or `data_structure.write()/try_write()` to lock the resource for read/write purposes.

`coordinates` holds x/y coordinates of the address (if `OSMGeoMapper::from_address` was used) or to the lat/lon initially provided. They are no longer in the original lat/lon format but in the data structure's coordinate system (each step is 100,000th of a degree, or roughly one meter at the equator).

`radius` is the chosen radius for the original fetching of data (if `OSMGeoMapper::from_address` or `OSMGeoMapper::from_lat_lon` was used).

If you wanted to get the GeoTile(s) at the real-world lat/lon of -75.6903082/45.4211063, you would use the following method call - `OSMGeoMapper::get_real(45.4211063, -75.6903082)`. Note that granularity is only to 6 decimal places. The method call above is the same as `OSMGeoMapper::get(45421106, -75690308)`

You can also get GeoTile(s) directly from the `OSMGeoMapper.data_structure` field (once read-locked) like this: `data_structure.get(geo_types::Coordinate { x: -7569031, y: 4542111 })`

You can convert to/from real and OSMGeoMapper coordinates using the following helper functions: `osm_geo_mapper::operations::to_tile_scale(f64) -> i32` and `osm_geo_mapper::operations::from_tile_scale(i32) -> f64`.

A GeoTile can be many many things - see `features.rs`.

You can also load more data into your `OSMGeoMapper.data_structure` using one of the following three helper methods:

    OSMGeoMapper::load_more_from_lat_lon(&mut self, latitude: f64, longitude: f64, radius: Option<u32>) -> Result<(), Box<dyn std::error::Error>>

    OSMGeoMapper::load_more_from_address(&mut self, address: String, radius: Option<u32>) -> Result<(), Box<dyn std::error::Error>>

    OSMGeoMapper::load_more_from_geojson_file(&mut self, geojson_file: String) -> Result<(), Box<dyn std::error::Error>>

See the `test_multiple_threads()` test function in `tests/lib_tests.rs` to see an example of loading data in multiple threads simultaneously.

TODO
====

- Implement logic to choose a lat/lon in the middle of a geojson file if none is provided via command line
- Implement (binary?) serialization of map data to be loaded/cached
- Continue tweaking the themes
- Need logic to handle rate-limited Overpass API calls (need to parse XML response)

ISSUES
======

Shift + Movement doesn't work in certain terminal emulators (confirmed in urxvt).
