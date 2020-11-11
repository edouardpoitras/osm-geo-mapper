extern crate osm_geo_mapper;

use osm_geo_mapper::{
    geo_types, address_to_mapper, lat_lon_to_mapper, geojson_file_to_mapper,
    features,
};

#[test]
fn test_address_to_mapper() {
    let mapper_result = address_to_mapper("ottawa ontario".to_string(), Some(20));
    assert!(mapper_result.is_ok());
    let mapper = mapper_result.unwrap();
    assert_eq!(mapper.coordinates, geo_types::Coordinate { x: -7569031, y: 4542111 });
    let some_unclassified = mapper.data_structure.get(&geo_types::Coordinate { x: -7569031, y: 4542111 });
    assert!(some_unclassified.is_some());
    let unclassified = some_unclassified.unwrap();
    let geotile_string = format!("{:?}", unclassified);
    let geotile_string_test = format!("{:?}", 
        features::GeoTile::Unclassified {
            geometry: features::Geometry::Point(
                geo_types::Point(
                    geo_types::Coordinate { x: -75.690308, y: 45.421106 }
                )
            ),
            osm_id: "\"node/18886011\"".to_string()
        }
    );
    assert_eq!(geotile_string, geotile_string_test);
    let some_building = mapper.data_structure.get(&geo_types::Coordinate { x: -7569021, y: 4542101 });
    assert!(some_building.is_some());
    let building = some_building.unwrap();
    assert!(matches!((*building).as_ref(), features::GeoTile::Building { .. }));
}