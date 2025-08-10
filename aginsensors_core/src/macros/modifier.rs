#[macro_export]
macro_rules! define_modifier {
    (
        $tag_value:literal,
        $struct_name:ident
    ) => {
        paste::paste! {
            #[derive(Clone, Debug)]
            pub struct $struct_name {
            }

        }
    };
}
