use aginsensors_core::{connector::Measurement, define_modifier, modifier::Modifier};
use color_eyre::eyre::Result;
use serde::de::value;

define_modifier!("modifier_template", ModifierTemplate);

impl Modifier for ModifierTemplate {
    fn calc(&self, measurements: Vec<Measurement>) -> Result<Vec<Measurement>> {
        // Implement the calculation logic here
        tracing::info!(
            "Calculating with ModifierTemplate for measurements: {:?}",
            measurements
        );

        let mut res = measurements.clone();
        res.iter_mut().for_each(|measurement| {
            measurement.values.iter_mut().for_each(|(_key, value)| {
                *value *= 2.0;
            });
        });

        tracing::info!("Calculated: {:?}", res);

        Ok(res)
    }
}
