#[test]
fn define_a_sstruct() {
    let mocha: Coffee = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true,
    };

    println!(
        "{} costs ${} and is hot: {}",
        mocha.name, mocha.price, mocha.is_hot
    );

    let fav_coffee = mocha.name;
    println!("fav_coffee: {}", fav_coffee);
}
#[derive(Debug)]
struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}

#[test]
fn overwrite_struct_field() {
    let mut beverage: Coffee = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true,
    };
    beverage.name = String::from("Caramel Machination");
    beverage.price = 1.0;
    beverage.is_hot = false;
    println!("beverage : {:?}", beverage);
}
#[test]
fn create_structs_in_a_function() {
    let name = String::from("Latte");
    let coffee = make_coffee(name, 4.99, true);
    println!("coffee : {:?}", coffee);
    println!(
        "My {} this morning cost {}. It is {} that it was hot.",
        coffee.name, coffee.price, coffee.is_hot
    );
}
fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name: name,
        price: price,
        is_hot: is_hot,
    }
}
fn make_coffee_shorthand(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot,
    }
}
#[test]
fn struct_field_initialization_shorthand_syntax() {
    let name = String::from("Latte");
    let price = 4.99;
    let is_hot = true;
    let coffee = make_coffee_shorthand(name, price, is_hot);
    println!("coffee : {:?}", coffee);
}
#[test]
fn struct_update_syntax() {
    let mocha: Coffee = make_coffee(String::from("Mocha"), 4.99, true);
    let caramel_machiato = Coffee {
        name: String::from("Caramel Machiato"),
        ..mocha
    };
    let macha = Coffee {
        name: mocha.name.clone(),
        ..mocha
    };
    println!("macha name: {}", macha.name);
    println!("mocha name: {}", mocha.name);
}
#[test]
fn passing_structs_into_a_function() {
    let mut mocha = make_coffee_shorthand(String::from("Mocha"), 4.99, false);
    drink_coffee(&mut mocha);
    println!("mocha : {}", mocha.is_hot);
}

fn drink_coffee(coffee: &mut Coffee) {
    println!("coffee: {}", coffee.name);
    println!("coffee: {:?}", coffee);
    coffee.is_hot = false;
}
#[test]
fn deriving_debug_trait_for_struct() {
    let value = ["hello", "world"];
    println!("{:?}", value);
    println!("{:#?}", value);

    let mocha = make_coffee_shorthand(String::from("Mocha"), 4.99, true);
    println!("{:?}", mocha);
    println!("{:#?}", mocha);

    #[allow(unused_variables)]
    let macha = make_coffee_shorthand(String::from("Machiato"), 4.99, true);
}
#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_sec: u32,
}

impl TaylorSwiftSong {
    /// This is an associated function, that not require the self parameter.
    /// this is equivalent as contractor in java.
    fn new(title: String, release_year: u32, duration_sec: u32) -> Self {
        Self {
            title,
            release_year,
            duration_sec,
        }
    }
}

// self -> imutable instance
// mut self -> mutable instance
// &self -> immutable reference to the instance
// &mut self -> mutable reference to the instance
impl TaylorSwiftSong {
    /*   /// This is an associated function, that not require the self parameter.
    /// this is equivalent as contractor in java.
    fn new(title: String, release_year: u32, duration_sec: u32) -> Self {
        Self{
            title,
            release_year,
            duration_sec,
        }
    }*/

    // immutable struct value (self parameter take ownership)
    // mutable struct value (self parameter take ownership, has permission to mutate )
    // immutable reference to the struct instance (no ownership moved)
    // Mutable reference to the struct instance( no ownership moved, have permission to mutate)
    fn display_song_info(self) {
        // immutable struct value (self parameter take ownership)
        println!("title: {}", self.title);
        println!("release_year: {}", self.release_year);
        println!("duration: {} secounds", self.duration_sec);
    }
    fn double_length(mut self) {
        // mutable struct value (self parameter take ownership, has permission to mutate )
        //self.duration_sec = self.duration_sec * 2;
        self.duration_sec *= 2;
        println!("duration_sec: {:#?}", self);
    }
    fn display_song_info_reference(&self) {
        // immutable reference to the struct instance (no ownership moved)
        println!("title: {}", self.title);
        println!("release_year: {}", self.release_year);
        println!("duration_sec: {}", self.duration_sec);
        println!("Years since release: {}", self.years_since_release());
    }
    fn double_length_reference(&mut self) {
        // Mutable reference to the struct instance( no ownership moved, have permission to mutate)
        self.duration_sec *= 2;
        println!("duration_sec: {:#?}", self);
    }
    fn is_longer_than(&self, other: &Self) -> bool {
        // method with multiple parameter in rust.
        self.duration_sec > other.duration_sec
    }
    fn years_since_release(&self) -> u32 {
        2026 - self.release_year
    }
}
#[test]
fn defining_struct_methods() {
    let song = TaylorSwiftSong {
        title: String::from("Love Story"),
        release_year: 2008,
        duration_sec: 235,
    };
    song.display_song_info();
}
#[test]
fn self_parameter_as_mutable_struct_instance() {
    let song = TaylorSwiftSong {
        title: String::from("Love Story"),
        release_year: 2008,
        duration_sec: 235,
    };
    // can not call this 2 function in the same time because we already lose the ownership of the song in the display function.
    //song.display_song_info();
    song.double_length();
    // can not call this either, because song no longer hold owner ship of that info.
    //println!("length: {}", song.duration_sec);
}
#[test]
fn self_parameter_as_immutable_reference_and_mutable_reference_to_struct_instance() {
    let mut song = TaylorSwiftSong {
        title: String::from("Love Story"),
        release_year: 2008,
        duration_sec: 235,
    };
    song.display_song_info_reference();
    // ownership was not taken away from song, so we can call this function again.
    println!("song title: {}", song.title);
    // ownership was not taken away from song, this mutable reference will work.
    song.double_length_reference();
    println!("song duration: {}", song.duration_sec);
}
#[test]
fn method_with_multiple_parameters() {
    let blank_space = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2014,
        duration_sec: 231,
    };
    let all_too_well = TaylorSwiftSong {
        title: String::from("All Too Well"),
        release_year: 2012,
        duration_sec: 330,
    };
    blank_space.is_longer_than(&all_too_well);
    println!("blank_space: {:?}", blank_space);
    println!("all too well: {:?}", all_too_well);
    println!(
        "is blank space longer than all too well? {}",
        blank_space.is_longer_than(&all_too_well)
    );

    if blank_space.is_longer_than(&all_too_well) {
        println!(
            "{} is longer than {}",
            blank_space.title, all_too_well.title
        );
    } else {
        println!(
            "{} is longer than {}",
            all_too_well.title, blank_space.title
        );
    }
}
#[test]
fn calling_methods_from_other_methods() {
    let blank_space = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2021,
        duration_sec: 235,
    };
    blank_space.display_song_info_reference();
}
#[test]
fn associated_functions() {
    let blank_space = TaylorSwiftSong::new(String::from("Blank Space"), 2021, 2022);
    println!("blank_space: {:?}", blank_space);
}
#[test]
fn multiple_impl_blocks() {
    let mut blank_space = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2021,
        duration_sec: 235,
    };
    blank_space.double_length_reference();
    println!("blank_space: {:?}", blank_space);
}
/// Builder design pattern
#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32,
}
impl Computer {
    fn new(cpu: String, memory: u32, hard_drive_capacity: u32) -> Computer {
        Self {
            cpu,
            memory,
            hard_drive_capacity,
        }
    }
    // implement the build design pattern, return self !
    fn upgrade_cpu(&mut self, new_cpu: String) -> &mut Self{
        self.cpu = new_cpu;
        self
    }
    fn upgrade_memory(&mut self, new_memory: u32) -> &mut Self {
        self.memory = new_memory;
        self
    }
    fn upgrade_hard_drive(&mut self, new_hard_drive_capacity: u32) -> &mut Self {
        self.hard_drive_capacity = new_hard_drive_capacity;
        self
    }
}
#[test]
fn builder_pattern() {
    let mut computer = Computer::new(String::from("M3 Max"), 64, 2);
    //using build design pattern here.
    computer.upgrade_cpu(String::from("M4 Max"))
        .upgrade_memory(128)
        .upgrade_hard_drive(4);
    println!("computer: {:#?}", computer);

}
