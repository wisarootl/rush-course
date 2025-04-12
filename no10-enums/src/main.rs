fn main() {
    intro_to_enums();
    enum_with_associated_values();
    struct_variants();
    nesting_enums_in_enums();
    the_match_keyword();
    the_match_keyword_1();
    defining_methods_on_enums();
    the_match_keyword_2();
    the_match_keyword_3();
    if_let_construct();
    let_else_construct();
    project();
}

#[derive(Debug)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

struct Card {
    rank: String,
    suit: CardSuit,
}

fn intro_to_enums() {
    println!("=== intro_to_enums ===");
    let first_card = CardSuit::Hearts;
    let mut second_card: CardSuit = CardSuit::Spades;
    println!("{:?}", second_card);
    second_card = CardSuit::Clubs;
    println!("{:?}", second_card);

    let card_suits = [CardSuit::Hearts, CardSuit::Clubs];
    println!("{:?}", card_suits);
    let card_suits = (CardSuit::Hearts, CardSuit::Diamonds);

    println!("{first_card:?}, {second_card:?}, {card_suits:?}");

    let hearts_ace = Card {
        rank: String::from("1"),
        suit: CardSuit::Hearts,
    };
    println!("{:?}, {:?}", hearts_ace.rank, hearts_ace.suit);
}

#[allow(dead_code)]
#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal(String, String),
}

fn enum_with_associated_values() {
    println!("=== enum_with_associated_values_i ===");
    let visa = PaymentMethodType::CreditCard(String::from("0034-5678-9012-3456"));
    let mastercard = PaymentMethodType::DebitCard(String::from("2532-1298-2093-4800"));
    println!("{:?}", visa);
    println!("{:?}", mastercard);

    let mut my_payment_method = PaymentMethodType::CreditCard(String::from("0034-5678-9012-3456"));

    println!("{:?}", my_payment_method);

    my_payment_method =
        PaymentMethodType::PayPal(String::from("bob@email.com"), String::from("password"));

    println!("{:?}", my_payment_method);
}

fn struct_variants() {
    println!("=== struct_variants ===");

    #[allow(dead_code)]
    #[derive(Debug)]
    enum PaymentMethodType {
        CreditCard(String),
        PayPal { username: String, password: String },
    }
    let visa = PaymentMethodType::CreditCard(String::from("0012-3456"));
    println!("{:?}", visa);

    let paypal = PaymentMethodType::PayPal {
        username: String::from("bob@gmail.com"),
        password: String::from("password"),
    };
    println!("{:?}", paypal);
}

fn nesting_enums_in_enums() {
    println!("=== nesting_enums_in_enums ===");
    #[derive(Debug)]
    enum Beans {
        Pinto,
        Black,
    }

    #[derive(Debug)]
    enum Meat {
        Chicken,
        Steak,
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    enum RestaurantItem {
        Burrito { meat: Meat, beans: Beans },
        Bowl { meat: Meat, beans: Beans },
        VeganPlate,
    }

    let lunch = RestaurantItem::Burrito {
        meat: Meat::Steak,
        beans: Beans::Pinto,
    };
    let dinner = RestaurantItem::Bowl {
        meat: Meat::Chicken,
        beans: Beans::Black,
    };
    let abandoned_meal = RestaurantItem::VeganPlate;
    println!("Lunch was {lunch:?} and dinner was {dinner:?}");
    println!("Nobody ate {abandoned_meal:?}");
}

#[allow(dead_code)]
#[derive(Debug)]
enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
}

fn the_match_keyword() {
    println!("=== the_match_keyword ===");
    let my_computer = OperatingSystem::MacOS;
    let age = _years_since_release(my_computer);
    println!("My computer's operating system is {age} years old");

    let dads_computer = OperatingSystem::Windows;
    let age = _years_since_release(dads_computer);
    println!("My dad's computer is {age} years old");
}

fn _years_since_release(os: OperatingSystem) -> u32 {
    match os {
        OperatingSystem::Windows => {
            println!("Quite an old operating system!");
            39
        }
        OperatingSystem::MacOS => 20,
        OperatingSystem::Linux => 25,
    }
}
enum LaundryCycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String),
}

fn the_match_keyword_1() {
    println!("=== the_match_keyword_1 ===");
    _wash_laundry(LaundryCycle::Cold);
    _wash_laundry(LaundryCycle::Hot { temperature: 100 });
    _wash_laundry(LaundryCycle::Delicate(String::from("Silk")));
}

fn _wash_laundry(cycle: LaundryCycle) {
    match cycle {
        LaundryCycle::Cold => {
            println!("Running the laundry with cold temperature")
        }

        // temperature is fixed name from enum definition
        LaundryCycle::Hot { temperature } => {
            println!("Running the laundry with a temperature of {temperature}");
        }

        // fabric_type_xxx is variable name
        LaundryCycle::Delicate(fabric_type_xxx) => {
            println!("Running the laundry with a delicate cycle for {fabric_type_xxx}");
        }
    }
}

impl LaundryCycle {
    fn wash_laundry(&self) {
        match self {
            LaundryCycle::Cold => {
                println!("Running the laundry with cold temperature")
            }
            LaundryCycle::Hot { temperature } => {
                println!("Running the laundry with a temperature of {temperature}");
            }
            LaundryCycle::Delicate(fabric_type) => {
                println!("Running the laundry with a delicate cycle for {fabric_type}");
            }
        }
    }
}

fn defining_methods_on_enums() {
    println!("=== defining_methods_on_enums ===");
    LaundryCycle::Cold.wash_laundry();
    let hot_cycle = LaundryCycle::Hot { temperature: 100 };
    hot_cycle.wash_laundry();

    let delicate_cycle = LaundryCycle::Delicate(String::from("Silk"));
    delicate_cycle.wash_laundry();
}

#[allow(dead_code)]
#[derive(Debug)]
enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered,
}

impl OnlineOrderStatus {
    fn check(&self) {
        match self {
            OnlineOrderStatus::Ordered | OnlineOrderStatus::Packed => {
                println!("Your item is being prepared for shipment")
            }
            OnlineOrderStatus::Delivered => {
                println!("Your item has arrived")
            }
            other_status => {
                println!("Your item is {other_status:?}")
            } //
              // _ => {
              //     println!("Your item is ...")
              // }
        }
    }
}

fn the_match_keyword_2() {
    println!("=== the_match_keyword_2 ===");
    OnlineOrderStatus::Ordered.check();
    OnlineOrderStatus::Delivered.check();
    OnlineOrderStatus::Shipped.check();
}

enum Milk {
    Lowfat(i32),
    Whole,
}

impl Milk {
    fn drink(self) {
        match self {
            Milk::Lowfat(2) => {
                println!("Delicious, 2% is my favorite!");
            }
            Milk::Lowfat(percent) => {
                println!("You've got the lowfat {percent} percent version!");
            }
            Milk::Whole => {
                println!("You've got the whole milk!");
            }
        }
    }
}

fn the_match_keyword_3() {
    println!("=== the_match_keyword_3 ===");
    Milk::Lowfat(2).drink();
    Milk::Lowfat(1).drink();
    Milk::Whole.drink();
}

enum Milk2 {
    Lowfat(i32),
    Whole,
    NonDairy { kind: String },
}

fn if_let_construct() {
    println!("=== if_let_construct ===");
    let my_beverage = Milk2::Whole;

    if let Milk2::Whole = my_beverage {
        println!("You have the Whole milk");
    }

    let my_beverage = Milk2::Lowfat(3);

    if let Milk2::Lowfat(percent) = my_beverage {
        println!("Your beverage is {percent}% milk.");
    }

    let my_beverage = Milk2::NonDairy {
        kind: String::from("oat"),
    };

    if let Milk2::NonDairy { kind } = my_beverage {
        println!("{kind} milk");
    }
}

fn let_else_construct() {
    println!("=== let_else_construct ===");
    let my_beverage = Milk2::NonDairy {
        kind: String::from("Oat"),
    };

    let Milk2::NonDairy { kind } = my_beverage else {
        println!("You do not have the nondairy milk");
        return;
    };

    println!("{kind} milk is available here");
}

#[allow(dead_code)]
#[derive(Debug)]
enum Tier {
    Gold,
    Silver,
    Platinum,
}

#[derive(Debug)]
enum Subscription {
    Free,
    Basic(f64, u32),
    Premium { tier: Tier },
}

impl Subscription {
    fn summarize(&self) {
        match self {
            Subscription::Free => {
                println!("You have limited access to the site");
            }
            Subscription::Basic(price, month) => {
                println!(
                    "You have limited access to the site's premium features for {price} for {month} months"
                );
            }
            Subscription::Premium { tier } => {
                println!(
                    "You have full access to the site's premium features. Your tier is {tier:?}"
                );
            }
        }
    }
}

fn project() {
    /*
    Define a Tier enum with three variants: Gold, Silver,
    Platinum. Derive a Debug implementation for the Tier enum.

    Define a Subscription enum with three variants: Free,
    Basic, and Premium. Derive a Debug implementation for the
    Subscription enum.

    The Free variant should have no associated data.

    The Basic variant should be a tuple variant with two pieces
    of data. The first one should be a f64 (the price per month)
    and the second one should be a u32 (the number of months).

    The Premium variant should be a struct variant with a 'tier'
    field. The tier field should be a Tier enum value.

    Define a 'summarize' method on the Subscription enum.

    If the Subscription is Free, output the text "You have
    limited access to the site".

    If the Subscription is Basic, output the text "You have
    limited access to the site's premium features for {price}
    for {months} months", where {price} amd {months} come from
    the tuple variant's associated data.

    If the Subscription is Premium, output the text "You have
    full access to the site's premium features. Your tier is
    {tier:?}"", where {tier} comes from the struct variant's
    associated 'tier' field.
    
    In the `main` function, create 3 instances of Subscription,
    each one with a different variant. Invoke the `summarize`
    method on each instance.
    */

    println!("=== project ===");
    Subscription::Free.summarize();

    let basic = Subscription::Basic(4.99, 3);
    basic.summarize();

    let premium = Subscription::Premium {
        tier: Tier::Platinum,
    };
    premium.summarize();
}
