use crate::tax::traits::{Investment, Taxable};

#[derive(Debug)]
pub struct Bonus {
    pub value: f64,
}

impl Investment<f64> for Bonus {
    fn amount(&self) -> f64 {
        self.value
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.value = new_amount
    }

    fn double_amount(&mut self) {
        self.value *= 2.0
    }
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.50;
}
