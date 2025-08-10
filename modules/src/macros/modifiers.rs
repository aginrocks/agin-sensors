#[macro_export]
macro_rules! define_modifiers {
    ($($path:ident::$name:ident),*) => {
        paste::paste! {
            #[derive(Clone, Debug)]
            pub enum ModifierType {
                $($name($path::$name)),*
            }
        }
    };
}
