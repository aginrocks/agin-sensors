use crate::databases::GlobalDB;

pub trait IntoGlobalDB {
    fn into_global_db(self) -> GlobalDB;
}
