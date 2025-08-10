#[macro_export]
macro_rules! define_modifier {
    (
        $tag_value:literal,
        $struct_name:ident
    ) => {
        paste::paste! {
            #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
            #[serde(rename = $tag_value)]
            pub struct $struct_name(String);

        }
    };
}
