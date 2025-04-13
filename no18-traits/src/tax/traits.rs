pub trait Investment<T> {
    fn amount(&self) -> T;

    fn set_amount(&mut self, new_amount: T);

    fn double_amount(&mut self) {
        // self.set_amount(self.amount() * 2.0);
    }
}

pub trait Taxable: Investment<f64> {
    // Taxable is inherited from Investment
    const TAX_RATE: f64 = 0.07;

    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}
