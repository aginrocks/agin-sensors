#[macro_export]
macro_rules! define_database {
    (
        $tag_value:literal,
        $struct_name:ident,
        global_config = { $($global_config:tt)* },
        global_state = { $($global_state:tt)* },
        local_config = { $($local_config:tt)* }
    ) => {
        paste::paste! {
            #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug)]
            pub struct [<LocalConfig$struct_name>] {
                 pub r#type: [<DatabaseType$struct_name>],

                 pub name: String,

                 $($local_config)*
            }

            #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug)]
            pub struct [<GlobalConfig$struct_name>] {
                 pub r#type: [<DatabaseType$struct_name>],

                 $($global_config)*
            }

            #[derive(Clone, Debug)]
            pub struct [<Global$struct_name>] {
                 $($global_state)*
            }

            #[derive(Clone, Debug)]
            pub struct [<Local$struct_name>] {
                pub config: [<LocalConfig$struct_name>],

                pub global: [<Global$struct_name>],
            }

            #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug)]
            pub enum [<DatabaseType$struct_name>] {
                #[serde(rename = $tag_value)]
                Value,
            }

            pub trait [<IntoGlobal$struct_name>] {
                fn into_global_db(self) -> [<Global$struct_name>];
            }
        }
    };
}
