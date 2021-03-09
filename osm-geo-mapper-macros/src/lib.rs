/// Helper macro to create the many enums used in this library.
#[macro_export]
macro_rules! create_enum {
    ($($enum:ident [$($variant:ident),*$(,)*]),*$(,)*) => {
        $(
            #[derive(Debug, Clone, Copy)]
            pub enum $enum {
                $(
                    $variant,
                )*
            }
        )*
    }
}

/// You can find all features at https://wiki.openstreetmap.org/wiki/Map_Features
#[macro_export]
macro_rules! implement_geotile {
    ($($variant:ident [$($attr:ident),*]),*$(,)*) => {
        paste! {
           // First generate the giant enum of GeoTiles.
           #[derive(Debug, Clone)]
           pub enum GeoTile {
               $(
                   $variant {
                       [<$variant:snake _type>]: [<$variant:camel Type>],
                       geometry: Geometry,
                       osm_id: String,
                       address: Option<Address>,
                       $(
                           $attr: Option<String>,
                       )*
                   },
               )*
           }
           // Now generate the display implementation.
           impl fmt::Display for GeoTile {
               fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                   match self {
                       $(
                           GeoTile::$variant {
                               $(
                                   $attr,
                               )*
                               [<$variant:snake _type>],
                               ..
                           } => {
                               let variant_str = stringify!($variant);
                               write!(f, "Feature: {}\n", variant_str)?;
                               write!(f, "Type: {:?}\n", [<$variant:snake _type>])?;
                               print_geotile_attributes!(f => $($attr),*);
                               Ok(())
                           },
                       )*
                   }
               }
            }
        }
    }
}

///
/// Helper macro to pretty-print all the Option<String> attributes of a particular GeoTile variant.
/// print_geotile_attributes!(f => field1, field2, ...)
/// Where f is a &mut fmt::Formatter (available when implementing fmt::Display).
/// 
#[macro_export]
macro_rules! print_geotile_attributes {
    ($f:expr => $($attr: ident),*$(,)*) => {
        {
            $(
                // Extract every Option<String> attribute and print it's value.
                if let Some($attr) = $attr {
                    let mut attr_str = stringify!($attr).to_string();
                    // Replace underscores with spaces and capitalize.
                    attr_str = attr_str.replace("_", " ");
                    let mut c = attr_str.chars();
                    attr_str = match c.next() {
                        None => String::new(),
                        Some(x) => x.to_uppercase().collect::<String>() + c.as_str(),
                    };
                    // Add our print statement.
                    write!($f, "{}: {}\n", attr_str, $attr)?;
                }
            )*
        }
    };
}


#[macro_export]
macro_rules! extract_type_from_string {
    ($type_str:ident<$props:ident> => $enum:ident [$($variant:ident),*$(,)*]) => {
        paste! {
            match $type_str {
                $(
                    stringify!([<$variant:snake>]) => $enum::$variant,
                )*
                _ => {
                    warn!("Unclassified {} {}: {:?}", stringify!($enum), $type_str, $props);
                    $enum::Unclassified
                }
            }
        }
    }
}

#[macro_export]
macro_rules! geotile_from_properties {
    ($geometry:ident<$props:ident> => $geotile_type:ident<$type:ident> [$($property:ident),*$(,)*]) => {
        let address = address_from_properties($props);
        let osm_id = $props["id"].to_string();
        $(
            let $property = property_to_option_string($props, stringify!($property));
        )*
        return GeoTile::$geotile_type {
            osm_id,
            $geometry,
            $type,
            address,
            $(
                $property,
            )*
        }
    }
}