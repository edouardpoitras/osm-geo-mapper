OSM Geo Mapper
==============

Navigate OpenStreetMap data in the terminal.
Will fetch OSM data using the latitude/longitude provided, convert to GeoJSON (requires dependencies), and display the resulting lines/points/polygons in the terminal.

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

This library isn't complete yet. There are OSM features/properties/details that I haven't encountered yet and that could result in the program panicking (by design for now). If you encounter such a case, please submit a issue with the address or geojsonfile attempted and I'll fix those issues as they arise.

Usage
=====

## CLI

    ./osm-geo-mapper --help

    ./osm-geo-mapper --address "110 laurier avenue west ottawa ontario"

## Library

    TBD

TODO
====

- Implement logic to choose a lat/lon in the middle of a geojson file if none is provided via command line
- Implement lazy-loading of more map data
- Implement (binary?) serialization of map data to be loaded/cached
- Add short-cut key info pane under details pane
- Continue adding missing properties/details/themes