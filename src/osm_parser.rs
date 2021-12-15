use std::fs::File;
use osm_xml;


pub fn parse_osm_file(filename: String) -> Result<osm_xml::OSM, osm_xml::error::Error> {
    let file = File::open(filename).unwrap();
    osm_xml::OSM::parse(file)
}