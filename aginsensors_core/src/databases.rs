use crate::define_databases;

pub mod influx;

define_databases!(influx::Influx);
