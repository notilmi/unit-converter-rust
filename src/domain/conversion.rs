#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Conversion {
    pub multiplier: f64,
}

impl Conversion {
    pub fn new(multiplier: f64) -> Self {
        Self { multiplier }
    }

    pub fn convert(&self, value: f64) -> f64 {
        value * self.multiplier
    }

    pub fn inverse(&self) -> Self {
        Self {
            multiplier: 1.0 / self.multiplier,
        }
    }
}
