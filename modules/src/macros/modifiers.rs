#[macro_export]
macro_rules! define_modifiers {
    ($($path:ident::$name:ident),*) => {
        paste::paste! {
            #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
            #[serde(untagged)]
            pub enum ModifierType {
                $($name($path::$name)),*
            }

            impl aginsensors_core::modifier::Modifier for ModifierType {
                fn calc(&self, measurements: Vec<aginsensors_core::connector::Measurement>) -> color_eyre::eyre::Result<Vec<aginsensors_core::connector::Measurement>> {
                    match self {
                        $(ModifierType::$name(modifier) => modifier.calc(measurements)),*
                    }
                }
            }

        }
    };
}
