use crate::tax::traits::{Investment, Taxable};

#[derive(Debug)]
pub struct Income {
    pub amount: f64,
}

impl Investment<f64> for Income {
    fn amount(&self) -> f64 {
        self.amount
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.amount = new_amount;
    }

    fn double_amount(&mut self) {
        self.amount *= 2.0
    }
}

impl Taxable for Income {}
