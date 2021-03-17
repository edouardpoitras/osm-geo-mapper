use crate::features::{BuildingType, GeoTile, HighwayType, LanduseType, LeisureType, ManMadeType, NaturalType};
use tui::style::{Color, Style};

pub struct GeoTileTheme {
    pub character: char,
    pub style: Style,
}

pub fn get_geo_tile_theme(geo_tile: &GeoTile) -> GeoTileTheme {
    match geo_tile {
        GeoTile::Aerialway { .. } => GeoTileTheme {
            character: 'A',
            style: Style::default().bg(Color::LightBlue),
        },
        GeoTile::Aeroway { .. } => GeoTileTheme {
            character: 'A',
            style: Style::default().fg(Color::LightBlue),
        },
        GeoTile::Amenity { .. } => GeoTileTheme {
            character: ' ',
            style: Style::default().bg(Color::LightMagenta),
        },
        GeoTile::Barrier { .. } => GeoTileTheme {
            character: 'B',
            style: Style::default().fg(Color::Red),
        },
        GeoTile::Boundary { .. } => GeoTileTheme {
            character: 'B',
            style: Style::default().fg(Color::LightCyan),
        },
        GeoTile::Building { building_type, .. } => match building_type {
            BuildingType::Hospital => GeoTileTheme {
                character: 'H',
                style: Style::default().bg(Color::DarkGray).fg(Color::Red),
            },
            BuildingType::FireStation => GeoTileTheme {
                character: 'F',
                style: Style::default().bg(Color::DarkGray).fg(Color::Red),
            },
            BuildingType::Cathedral | BuildingType::Chapel | BuildingType::Church => GeoTileTheme {
                character: 'C',
                style: Style::default().bg(Color::DarkGray).fg(Color::White),
            },
            BuildingType::Mosque => GeoTileTheme {
                character: 'M',
                style: Style::default().bg(Color::DarkGray).fg(Color::White),
            },
            BuildingType::Religious => GeoTileTheme {
                character: 'R',
                style: Style::default().bg(Color::DarkGray).fg(Color::White),
            },
            BuildingType::Shrine | BuildingType::Synagogue => GeoTileTheme {
                character: 'S',
                style: Style::default().bg(Color::DarkGray).fg(Color::White),
            },
            BuildingType::Temple => GeoTileTheme {
                character: 'T',
                style: Style::default().bg(Color::DarkGray).fg(Color::White),
            },
            _ => {
                let c: char = format!("{:?}", building_type).chars().next().unwrap();
                GeoTileTheme {
                    character: c,
                    style: Style::default().bg(Color::DarkGray),
                }
            }
        },
        GeoTile::Craft { .. } => GeoTileTheme {
            character: 'C',
            style: Style::default().bg(Color::DarkGray),
        },
        GeoTile::Emergency { .. } => GeoTileTheme {
            character: 'E',
            style: Style::default().bg(Color::LightRed),
        },
        GeoTile::Geological { .. } => GeoTileTheme {
            character: 'G',
            style: Style::default(),
        },
        GeoTile::Healthcare { .. } => GeoTileTheme {
            character: 'H',
            style: Style::default(),
        },
        GeoTile::Highway { highway_type, .. } => match highway_type {
            HighwayType::Track => GeoTileTheme {
                character: 'T',
                style: Style::default().bg(Color::White),
            },
            HighwayType::BusStop => GeoTileTheme {
                character: 'B',
                style: Style::default().bg(Color::White),
            },
            HighwayType::Crossing => GeoTileTheme {
                character: 'C',
                style: Style::default().bg(Color::White),
            },
            HighwayType::Stop => GeoTileTheme {
                character: 'S',
                style: Style::default().bg(Color::White),
            },
            HighwayType::StreetLamp => GeoTileTheme {
                character: 'L',
                style: Style::default().bg(Color::White),
            },
            HighwayType::TrafficSignals => GeoTileTheme {
                character: 'S',
                style: Style::default().bg(Color::White),
            },
            _ => GeoTileTheme {
                character: ' ',
                style: Style::default().bg(Color::White),
            },
        },
        GeoTile::Historic { .. } => GeoTileTheme {
            character: 'H',
            style: Style::default().bg(Color::Yellow),
        },
        GeoTile::Landuse { landuse_type, .. } => match landuse_type {
            LanduseType::Farmland | LanduseType::Farmyard | LanduseType::Forest | LanduseType::Grass | LanduseType::Greenfield => GeoTileTheme {
                character: ' ',
                style: Style::default().bg(Color::Green),
            },
            _ => GeoTileTheme {
                character: ' ',
                style: Style::default().bg(Color::LightGreen),
            }
        },
        GeoTile::Leisure { leisure_type, .. } => match leisure_type {
            LeisureType::DogPark | LeisureType::MiniatureGolf | LeisureType::NatureReserve | LeisureType::Park => GeoTileTheme {
                character: ' ',
                style: Style::default().bg(Color::Green),
            },
            _ => GeoTileTheme {
                character: ' ',
                style: Style::default().bg(Color::Cyan),
            }
        },
        GeoTile::ManMade { man_made_type, .. } => match man_made_type {
            ManMadeType::Bridge => GeoTileTheme {
                character: 'B',
                style: Style::default(),
            },
            _ => GeoTileTheme {
                character: 'M',
                style: Style::default(),
            }
        },
        GeoTile::Military { .. } => GeoTileTheme {
            character: 'M',
            style: Style::default().fg(Color::Red),
        },
        GeoTile::Natural { natural_type, .. } => match natural_type {
            NaturalType::Tree | NaturalType::TreeRow | NaturalType::Wood => GeoTileTheme {
                character: 'T',
                style: Style::default().fg(Color::Green),
            },
            NaturalType::Arete => GeoTileTheme {
                character: 'A',
                style: Style::default(),
            },
            NaturalType::BareRock
            | NaturalType::CaveEntrance
            | NaturalType::Cliff
            | NaturalType::Stone
            | NaturalType::Ridge
            | NaturalType::Rock => GeoTileTheme {
                character: 'R',
                style: Style::default(),
            },
            NaturalType::Bay | NaturalType::Beach | NaturalType::Blowhole => GeoTileTheme {
                character: 'B',
                style: Style::default().bg(Color::LightYellow),
            },
            NaturalType::Saddle
            | NaturalType::Sand
            | NaturalType::Scree
            | NaturalType::Scrub
            | NaturalType::Shingle
            | NaturalType::Sinkhole
            | NaturalType::Spring
            | NaturalType::Strait => GeoTileTheme {
                character: 'S',
                style: Style::default(),
            },
            NaturalType::Valley | NaturalType::Volcano => GeoTileTheme {
                character: 'V',
                style: Style::default().bg(Color::Gray),
            },
            NaturalType::Water | NaturalType::Wetland => GeoTileTheme {
                character: 'W',
                style: Style::default().bg(Color::Blue),
            },
            _ => GeoTileTheme {
                character: 'N',
                style: Style::default().fg(Color::Green),
            },
        },
        GeoTile::Office { .. } => GeoTileTheme {
            character: 'O',
            style: Style::default().bg(Color::LightMagenta),
        },
        GeoTile::Place { .. } => GeoTileTheme {
            character: 'P',
            style: Style::default().bg(Color::Black).fg(Color::White),
        },
        GeoTile::Power { .. } => GeoTileTheme {
            character: 'P',
            style: Style::default().fg(Color::LightYellow),
        },
        GeoTile::PublicTransport { .. } => GeoTileTheme {
            character: 'P',
            style: Style::default(),
        },
        GeoTile::Railway { .. } => GeoTileTheme {
            character: 'R',
            style: Style::default().bg(Color::LightMagenta),
        },
        GeoTile::Route { .. } => GeoTileTheme {
            character: 'R',
            style: Style::default().fg(Color::LightMagenta),
        },
        GeoTile::Shop { .. } => GeoTileTheme {
            character: 'S',
            style: Style::default().bg(Color::LightGreen),
        },
        GeoTile::Sport { .. } => GeoTileTheme {
            character: 'S',
            style: Style::default().fg(Color::LightCyan),
        },
        GeoTile::Telecom { .. } => GeoTileTheme {
            character: 'T',
            style: Style::default().fg(Color::LightBlue),
        },
        GeoTile::Tourism { .. } => GeoTileTheme {
            character: 'T',
            style: Style::default().bg(Color::LightYellow),
        },
        GeoTile::Water { .. } => GeoTileTheme {
            character: ' ',
            style: Style::default().bg(Color::Blue),
        },
        GeoTile::Waterway { .. } => GeoTileTheme {
            character: ' ',
            style: Style::default().bg(Color::Blue),
        },
        GeoTile::Unclassified { .. } => GeoTileTheme {
            character: 'U',
            style: Style::default(),
        },
    }
}
