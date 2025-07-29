use crate::define_databases;

mod influx;

define_databases!(influx::Influx);
