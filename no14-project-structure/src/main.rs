mod inventory {
    const FLOOR_SPACE: i32 = 10000;
    const MANAGER: &str = "John Doe";

    #[derive(Debug)]
    enum ProductCategory {
        Ladder,
        Hammer,
    }

    #[derive(Debug)]
    struct Item {
        name: String,
        category: ProductCategory,
        quantity: u32,
    }

    fn talk_to_managet() {
        println!("Hey, {MANAGER}, how's your coffee?")
    }
}

fn main() {
    println!("Hello, world!");
}
