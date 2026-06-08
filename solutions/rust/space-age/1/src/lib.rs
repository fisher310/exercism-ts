#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / 31557600.0
    }
}

pub struct Mercury(f64);

pub struct Venus(f64);

pub struct Earth(f64);

pub struct Mars(f64);

pub struct Jupiter(f64);

pub struct Saturn(f64);

pub struct Uranus(f64);

pub struct Neptune(f64);

impl Default for Mercury {
    fn default() -> Self {
        Self(0.2408467)
    }
}

impl Default for Venus {
    fn default() -> Self {
        Self(0.61519726)
    }
}

impl Default for Earth {
    fn default() -> Self {
        Self(1.0)
    }
}

impl Default for Mars {
    fn default() -> Self {
        Self(1.8808158)
    }
}

impl Default for Jupiter {
    fn default() -> Self {
        Self(11.862615)
    }
}

impl Default for Saturn {
    fn default() -> Self {
        Self(29.447498)
    }
}

impl Default for Uranus {
    fn default() -> Self {
        Self(84.016846)
    }
}

impl Default for Neptune {
    fn default() -> Self {
        Self(164.79132)
    }
}

macro_rules! impl_planet {
    (for $($T: ty),+) => {
        $(impl Planet for $T {
            fn years_during(d: &Duration) -> f64 {
                let t = <$T>::default();
                d.seconds / 31557600.0 / t.0
            }
        })*
    }
}

impl_planet!(for Mercury, Earth, Venus, Mars, Jupiter, Saturn, Uranus, Neptune );