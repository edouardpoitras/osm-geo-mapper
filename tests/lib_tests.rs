extern crate osm_geo_mapper;
use std::thread;
use osm_geo_mapper::{
    geo_types, interface, features
};

#[test]
fn test_address_to_mapper() {
    let mapper_result = interface::OSMGeoMapper::from_address("ottawa ontario".to_string(), Some(20));
    assert!(mapper_result.is_ok());
    let mapper1 = mapper_result.unwrap();
    let mapper2 = mapper1.atomic_clone();
    assert_eq!(mapper1.coordinates, geo_types::Coordinate { x: -7569006, y: 4542114 });
    let locked_data_structure1 = mapper1.data_structure.read().unwrap();
    let some_place = locked_data_structure1.get(&geo_types::Coordinate { x: -7569006, y: 4542114 });
    assert!(some_place.is_some());
    let unclassified = some_place.unwrap();
    let geotile_string = format!("{:?}", unclassified);
    let geotile_string_test = format!("{:?}", vec![
        features::GeoTile::Place {
            address: None,
            admin_level: Some("2".to_string()),
            architect: None,
            capital: Some("yes".to_string()),
            geometry: features::Geometry::Point(
                geo_types::Point(
                    geo_types::Coordinate { x: -75.6900574, y: 45.4211435 }
                )
            ),
            is_in: None,
            name: Some("Ottawa".to_string()),
            osm_id: "node/18886011".to_string(),
            place_type: features::PlaceType::City,
            population: Some("934243".to_string()),
            reference: None,
            start_date: None,
            state_code: None,
        }
    ]
    );
    assert_eq!(geotile_string, geotile_string_test);
    let locked_data_structure2 = mapper2.data_structure.read().unwrap();
    let some_building = locked_data_structure2.get(&geo_types::Coordinate { x: -7569006, y: 4542114 });
    assert!(some_building.is_some());
    let building = some_building.unwrap();
    let geotile: &features::GeoTile = building.last().unwrap().as_ref();
    match *geotile {
        features::GeoTile::Place { .. } => { },
        _ => assert!(false)
    }
}

#[test]
fn test_multiple_threads() {
    let mapper_result = interface::OSMGeoMapper::from_address("ottawa ontario".to_string(), Some(20));
    let mapper = mapper_result.unwrap();
    let mut threads = vec![];
    for address in vec!["ottawa ontario".to_string(), "montreal quebec".to_string()] {
        let mut mapper_clone = mapper.atomic_clone();
        threads.push(thread::spawn(move || {
            let result = mapper_clone.load_more_from_address(address, Some(20));
            match result {
                Err(_) => {}
                Ok(()) => {}
            }
        }));
    }
    for thread in threads {
        let _ = thread.join();
    }
    let geo_tile = mapper.get(4542114, -7569006).unwrap();
    let value = geo_tile.last().unwrap().as_ref();
    assert!(matches!(value, features::GeoTile::Place { .. }));
    let geo_tile = mapper.get_real(45.42114, -75.69006).unwrap();
    let value = geo_tile.last().unwrap().as_ref();
    assert!(matches!(value, features::GeoTile::Place { .. }));
}