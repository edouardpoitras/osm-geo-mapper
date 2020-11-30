extern crate osm_geo_mapper;

use osm_geo_mapper::{
    geo_types, interface, features
};

#[test]
fn test_address_to_mapper() {
    let mapper_result = interface::OSMGeoMapper::from_address("ottawa ontario".to_string(), Some(20));
    assert!(mapper_result.is_ok());
    let mapper = mapper_result.unwrap();
    assert_eq!(mapper.coordinates, geo_types::Coordinate { x: -7569031, y: 4542111 });
    let locked_data_structure = mapper.data_structure.read().unwrap();
    let some_place = locked_data_structure.get(&geo_types::Coordinate { x: -7569031, y: 4542111 });
    assert!(some_place.is_some());
    let unclassified = some_place.unwrap();
    let geotile_string = format!("{:?}", unclassified);
    let geotile_string_test = format!("{:?}", 
        features::GeoTile::Place {
            admin_level: Some("2".to_string()),
            architect: None,
            capital: Some("yes".to_string()),
            geometry: features::Geometry::Point(
                geo_types::Point(
                    geo_types::Coordinate { x: -75.690308, y: 45.421106 }
                )
            ),
            is_in: None,
            name: Some("Ottawa".to_string()),
            osm_id: "\"node/18886011\"".to_string(),
            place_type: features::PlaceType::City,
            population: Some("934243".to_string()),
            reference: None,
            start_date: None,
            state_code: None,
        }
    );
    assert_eq!(geotile_string, geotile_string_test);
    let some_building = locked_data_structure.get(&geo_types::Coordinate { x: -7569021, y: 4542101 });
    assert!(some_building.is_some());
    let building = some_building.unwrap();
    assert!(matches!((*building).as_ref(), features::GeoTile::Building { .. }));
}