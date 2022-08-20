use crate::exporter::Exporter;
use air_quality_messages::*;
use once_cell::sync::Lazy;
use paste::paste;
use prometheus::{self, register_gauge, Gauge};

#[derive(Default)]
/// Exporter for the 'particle' channel
pub struct ParticleExporter;

impl Exporter<Particles> for ParticleExporter {
    fn channel(&self) -> &str {
        "particle"
    }

    fn process(&self, msg: Particles) {
        PARTICLE_MASS_1_0.set(msg.pm1_0_mass.into());
        PARTICLE_MASS_2_5.set(msg.pm2_5_mass.into());
        PARTICLE_MASS_4_0.set(msg.pm4_0_mass.into());
        PARTICLE_MASS_10_0.set(msg.pm10_0_mass.into());
        PARTICLE_NUMBER_0_5.set(msg.pm0_5_number.into());
        PARTICLE_NUMBER_1_0.set(msg.pm1_0_number.into());
        PARTICLE_NUMBER_2_5.set(msg.pm2_5_number.into());
        PARTICLE_NUMBER_4_0.set(msg.pm4_0_number.into());
        PARTICLE_NUMBER_10_0.set(msg.pm10_0_number.into());
        PARTICLE_SIZE.set(msg.partical_size.into());
    }
}

// Static prometheus objects below

macro_rules! particle_gauge {
    ( $label:ident, $desc:expr ) => {
        paste! {
            pub static $label: Lazy<Gauge> =
                Lazy::new(|| register_gauge!(
                    stringify!([< $label:lower >]), $desc).unwrap());
        }
    };
}

particle_gauge! {PARTICLE_MASS_1_0, "Mass Concentration PM1.0"}
particle_gauge! {PARTICLE_MASS_2_5, "Mass Concentration PM2.5"}
particle_gauge! {PARTICLE_MASS_4_0, "Mass Concentration PM4.0"}
particle_gauge! {PARTICLE_MASS_10_0, "Mass Concentration PM10.0"}
particle_gauge! {PARTICLE_NUMBER_0_5, "Number Concentration PM0.5"}
particle_gauge! {PARTICLE_NUMBER_1_0, "Number Concentration PM1.0"}
particle_gauge! {PARTICLE_NUMBER_2_5, "Number Concentration PM2.5"}
particle_gauge! {PARTICLE_NUMBER_4_0, "Number Concentration PM4.0"}
particle_gauge! {PARTICLE_NUMBER_10_0, "Number Concentration PM10.0"}
particle_gauge! {PARTICLE_SIZE, "Typical Particle Size"}
