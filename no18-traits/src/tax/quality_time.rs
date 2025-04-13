use crate::tax::traits::Investment;

pub struct QualityTime {
    pub minutes: u32,
}

impl Investment<u32> for QualityTime {
    fn amount(&self) -> u32 {
        self.minutes
    }

    fn set_amount(&mut self, new_amount: u32) {
        self.minutes = new_amount
    }

    fn double_amount(&mut self) {
        self.minutes *= 2;
    }
}
