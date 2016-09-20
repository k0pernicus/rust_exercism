// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
#![allow(unused_variables)]

const EARTH_YEAR: f64 = 31_557_600.;
const MERCURY_YEAR: f64 = 0.2408467;
const VENUS_YEAR: f64 = 0.61519726;
const MARS_YEAR: f64 = 1.8808158;
const JUPITER_YEAR: f64 = 11.862615;
const SATURN_YEAR: f64 = 29.447498;
const URANUS_YEAR: f64 = 84.016846;
const NEPTUNE_YEAR: f64 = 164.79132;

pub struct Duration {
    earth_years: f64,
}

impl From<u64> for Duration {
    // Convert the duration in Earth years
    fn from(s: u64) -> Self {
        // An Earth year, in seconds, corresponds to EARTH_YEAR
        // To get the number of Earth years from seconds, just div...
        let earth_years: f64 = s as f64 / EARTH_YEAR;
        Duration { earth_years: earth_years }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        unimplemented!();
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / MERCURY_YEAR
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / VENUS_YEAR
    }
}

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years
    }
}

impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / MARS_YEAR
    }
}

impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / JUPITER_YEAR
    }
}

impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / SATURN_YEAR
    }
}

impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / URANUS_YEAR
    }
}

impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / NEPTUNE_YEAR
    }
}
