use traits::tax::{Bonus, Income, Investment, QualityTime, Taxable};

pub fn main() {
    println!("=== no05_associated_constants_in_a_trait ===");
    let mut income = Income { amount: 50000.50 };
    println!("Total tax owed: ${:.2}", income.tax_bill());
    income.double_amount();
    println!("Total tax owed: ${:.2}", income.tax_bill());

    let mut bonus = Bonus { value: 10000.23 };
    println!("Total tax owed: ${:.2}", bonus.tax_bill());
    bonus.double_amount();
    println!("Total tax owed: ${:.2}", bonus.tax_bill());

    let mut rust_programming_time = QualityTime { minutes: 1000 };
    rust_programming_time.double_amount();
    println!("Relaxation time: {:.2}", rust_programming_time.amount());
}
