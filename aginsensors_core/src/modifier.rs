use color_eyre::eyre::Result;

use crate::connector::Measurement;

pub trait Modifier {
    /// Runs calculations on data
    fn calc(&self, measurements: Vec<Measurement>) -> Result<Vec<Measurement>>;
}
