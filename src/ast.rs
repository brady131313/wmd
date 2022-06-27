#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeUnit {
    Second,
    Minute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Percent,
    Rep,
    Time(TimeUnit),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quantity(f64, Unit);

impl Quantity {
    pub fn new(value: f64, unit: Unit) -> Self {
        Self(value, unit)
    }
}
