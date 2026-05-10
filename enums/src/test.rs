#[derive(Debug)]
enum CardSuit {
    Diamond,
    Clubs,
    Hearts,
    Spades,
}

struct Card {
    suit: CardSuit,
    rank: String,
}
#[test]
fn into_to_enums() {
    let _first_card = CardSuit::Diamond;
    let mut secound_card = CardSuit::Hearts;
    println!("first_card: {:?}", _first_card);
    secound_card = CardSuit::Spades;
    println!("first_card: {:?}", secound_card);

    let _card_suit = [
        CardSuit::Diamond,
        CardSuit::Clubs,
        CardSuit::Hearts,
        CardSuit::Spades,
    ];
    let _card_suits = (CardSuit::Spades, CardSuit::Clubs);
}
#[derive(Debug)]
enum PathmentMethodType {
    CreditCard((String), (i32), (bool)),
    DebitCard((String), (i32), (bool)),
    Paypal((String), (i32), (bool)),
}
#[test]
fn enum_with_associated_values_1() {
    let visa = PathmentMethodType::CreditCard(String::from("0034-5678-9495-5613"), 1, false);
    let master_card = PathmentMethodType::DebitCard(String::from("0034-5678-9495-5613"), 1, false);
    println!("visa: {:#?}", visa);
    println!("master_card: {:#?}", master_card);
}
#[derive(Debug)]
enum PaymentMethodType2 {
    CreditCard(String, i32),
    DebitCard(String, bool),
    Paypal(String, String),
}
#[test]
fn enum_with_associated_values_2() {
    let mut my_payment_method = PaymentMethodType2::CreditCard(
        String::from("0034-5678-9495-5613"),
        1213 - 4546 - 4564 - 8989,
    );
    my_payment_method =
        PaymentMethodType2::Paypal(String::from("0034-5678-9495-5613"), String::from("123"));
    println!("my_payment_method: {:#?}", my_payment_method);
}
#[test]
fn a_brief_discussion_on_enum_memory() {
    //enum like pointer to heap addrest of type
    // address size in the heap will base on the largest require in enum
}
#[derive(Debug)]
enum PaymentMethodType3 {
    CreditCard(String, i32),
    DebitCard(String, bool),
    //Paypal(Credentials),
    Paypal {
        username: String,
        password: String,
    }
}
#[test]
fn struct_variants() {
    let visa = PaymentMethodType3::CreditCard(String::from("0034-5678-9495-5613"), 12);
    println!("visa: {:#?}", visa);

    let paypal = PaymentMethodType3::Paypal{
        username: String::from("user123"),
        password: String::from("password123")
    };
    println!("PayPal info: {:#?}", paypal);
}

#[derive(Debug)]
enum Beans{
    Pinto,
    Black,
}
#[derive(Debug)]
enum Meat{
    Chicken,
    Steak,
}
#[derive(Debug)]
enum RestaurantItem{
    Burrito(Meat),
    Bowl{
        meat: Meat,
        beans: Beans,
    },
    VeganPlate,
}
#[test]
fn nesting_enums_in_enums() {
    let lunch = RestaurantItem::Burrito(Meat::Steak);
    let dinner = RestaurantItem::Bowl{
        meat: Meat::Steak,
        beans: Beans::Pinto,
    };
    let abandon_meal = RestaurantItem::VeganPlate;
    println!("lunch was {:#?} and dinner was {:#?}", lunch,dinner);
    println!("abandon meal was {:#?}", abandon_meal);
}
enum OperatingSystem{
    Window,
    MacOS,
    Linux,
}
fn years_since_release(os : OperatingSystem) -> u32 {
    match os {
        OperatingSystem::Window => 20,
        OperatingSystem::MacOS => 10,
        OperatingSystem::Linux => 8,
        _ => panic!("Unknown operating system"),

    }

}
#[test]
fn the_mattch_keyword_I (){
    let my_computter = OperatingSystem::Linux;
    let a = years_since_release(my_computter);
    println!("My computer OS is {} uear old", a);
}
fn years_since_release_2(os : OperatingSystem) -> u32 {
    match os {
        OperatingSystem::Window => {
            println!("Quite old Operating System");
            20
        },
        OperatingSystem::MacOS => 10,
        OperatingSystem::Linux => 8,
        _ => panic!("Unknown operating system"),

    }

}
#[test]
fn the_mattch_keyword_II (){
    let my_computter = OperatingSystem::Window;
    let a = years_since_release_2(my_computter);
    println!("My computer OS is {} uear old", a);
}
fn wash_laundry(cycle: LandryCycle) {
    match cycle {
        LandryCycle::Cold => {
            println!("Washing with cold water")
        }
        LandryCycle::Hot {temperature} => {
            println!("Washing with hot water at {} degrees", temperature)
        }
        LandryCycle::Delicate (fabric) => {
            println!("Washing delicate fabric: {}", fabric)
        }
    }
}
enum LandryCycle {
    Cold,
    Hot {temperature: u32},
    Delicate (String),
}
impl LandryCycle {
    fn wash_laundry(&self) {
        match self {
            LandryCycle::Cold => {
                println!("Washing with cold water")
            }
            LandryCycle::Hot {temperature} => {
                println!("Washing with hot water at {} degrees", temperature)
            }
            LandryCycle::Delicate(fabric) => {
                println!("Washing delicate fabric: {}", fabric)
            }
        }
    }
}
#[test]
fn the_mattch_keyword_III(){
    wash_laundry(LandryCycle::Hot {temperature: 20});
    wash_laundry(LandryCycle::Cold);
    wash_laundry(LandryCycle::Delicate(String::from("Cotton")));
}

#[test]
fn defining_methods_on_enums() {
    LandryCycle::Cold.wash_laundry();
    LandryCycle::Hot{temperature: 30}.wash_laundry();
    LandryCycle::Delicate(String::from("Cotton")).wash_laundry();

    let delicate_cycle = LandryCycle::Delicate(String::from("Silk"));
    delicate_cycle.wash_laundry();
}
#[derive(PartialEq, Debug)]
enum OnlineOrderStatus {
    Order,
    Packed,
    Shipped,
    Delivered,
    Trask,
}
impl OnlineOrderStatus {
    fn check(&self){
        match self {
            OnlineOrderStatus::Delivered => {
                println!("Your order has been delivered. Enjoy your purchase!");
            }
            OnlineOrderStatus::Order | OnlineOrderStatus::Packed => {
                println!("Your order has been prepare for shipment.");
            }
            OnlineOrderStatus::Shipped => {
                println!("Your order has been shipped.");
            }
            other_status => {
                println!("Your order has been {:?}",other_status);
            }
        }
    }
}
#[test]
fn the_match_keyword_IV_catching_nultiple_values(){
    let order_status = OnlineOrderStatus::Shipped;
    order_status.check();

}

enum Milk{
    LowFat(i32),
    WholeMilk,
    NonDairy{kind: String},
}
impl Milk {
    fn drink(self) -> Milk {
        match  self {
            Milk::LowFat(2) => {
                println!("Drinking low-fat milk with 2% fat");
                Milk::LowFat(2)
            }
            Milk::LowFat(percent) => {
                println!("Drinking low-fat milk with {}% fat", percent);
                Milk::LowFat(percent)
            }
            Milk::WholeMilk => {
                println!("Drinking whole milk");
                Milk::WholeMilk
            }
            Milk::NonDairy{kind} => {
                println!("Drinking non-dairy milk of kind: {}", kind);
                Milk::NonDairy { kind }
            }
        }
    }
}
#[test]
fn the_match_keyword_V_match_with_exact_value(){
    Milk::LowFat(1).drink();
    Milk::WholeMilk.drink();
    Milk::LowFat(2).drink();
}
#[test]
fn the_if_let_construct(){
    // The if let construct combine an if statement with a variable  declaration.
    let my_beverage = Milk::WholeMilk;

    if let Milk::WholeMilk = my_beverage {
        println!("I am drinking whole milk");
    } else {
        println!("I am drinking something else");
    }

    let low_fat = Milk::LowFat(2);
    if let Milk::LowFat(percent) = low_fat {
        println!("I am drinking low-fat milk with {}% fat", percent);
    } else {
        println!("I am drinking something else");
    }

    let none_dariy = Milk::NonDairy{kind: String::from("none_dariy")};
    if let Milk::NonDairy{kind} = none_dariy {
        println!("I am drinking something else {}",kind);
    }else {
        println!("I am drinking something else");
    }
}

#[test]
fn the_let_else_construct() {
    let my_beverage = Milk::WholeMilk;

    // this else onlu execute when the my_beverage is not lowfar.
    let Milk::LowFat(percent) = my_beverage else {
        println!("I am not drinking low-fat milk");
        return;// need to use return to terminate if the percentage is not available.
    };
    println!("{percent}% milk is available here");

    let Milk::NonDairy {kind} = my_beverage else {
        println!("I am not drinking non-dairy milk");
        return;
    };
    println!("I am drinking non-dairy milk of kind: {}", kind);
}
#[derive(Debug)]
enum Tier{
    Gold,
    Silver,
    Platinum,
}
#[derive(Debug)]
enum Subcription {
    Free,
    Basic (f64,u32),
    Premium {
        tier: Tier,
    },
}
impl Subcription{
    fn summarize (&self){
        match self {
            Subcription::Free => {
                println!("You have a limited access to the site.");
            }
            Subcription::Basic(price,month) => {
                println!("You have limited access to the site's premium features for {} for {}",price,month);
            }
            Subcription::Premium{tier} => {
                println!("You have full access to the site's premium feature.\n\
                Your tier is {:?}",tier);
            }
        }
    }
}
#[test]
fn project_section_10 (){
    Subcription::Free.summarize();
    Subcription::Basic(9.99, 6).summarize();
    Subcription::Premium{tier: Tier::Gold}.summarize();
}