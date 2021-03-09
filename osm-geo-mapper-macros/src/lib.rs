#[macro_export]
macro_rules! extract_type_from_string {
    ($type_str:ident<$props:ident> => $($enum:ident [$($variant:ident),*$(,)*]),*$(,)*) => {
        paste! {
            $(
                match $type_str {
                    $(
                        stringify!([<$variant:snake>]) => $enum::$variant,
                    )*
                    _ => {
                        warn!("Unclassified {} {}: {:?}", stringify!($enum), $type_str, $props);
                        $enum::Unclassified
                    }
                }
            )*
        }
    }
}
