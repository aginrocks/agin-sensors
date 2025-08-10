#[macro_export]
macro_rules! define_modifiers {
    ($($path:ident::$name:ident),*) => {
        paste::paste! {
            #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
            #[serde(untagged)]
            pub enum ModifierType {
                $($name($path::$name)),*
            }

        }
    };
}
