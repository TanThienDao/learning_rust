

#[test]
fn define_a_sstruct(){
    let mocha: Coffee = Coffee{
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true,
    };

    println!(
        "{} costs ${} and is hot: {}",
        mocha.name,
        mocha.price,
        mocha.is_hot
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
fn overwrite_struct_field (){
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
fn create_structs_in_a_function(){
    let name = String::from("Latte");
    let coffee = make_coffee(name, 4.99, true);
    println!("coffee : {:?}", coffee);
    println!("My {} this morning cost {}. It is {} that it was hot.", coffee.name,coffee.price,coffee.is_hot);
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
fn struct_field_initialization_shorthand_syntax(){
    let name = String::from("Latte");
    let price = 4.99;
    let is_hot = true;
    let coffee = make_coffee_shorthand(name, price, is_hot);
    println!("coffee : {:?}", coffee);

}
#[test]
fn struct_update_syntax() {
    let mocha: Coffee = make_coffee(String::from("Mocha"), 4.99, true);
    let caramel_machiato = Coffee{
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
fn passing_structs_into_a_function(){
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
fn deriving_debug_trait_for_struct (){
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

// self -> imutable instance
// mut self -> mutable instance
// &self -> immutable reference to the instance
// &mut self -> mutable reference to the instance
impl TaylorSwiftSong {
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
    }
    fn double_length_reference(&mut self) {
        // Mutable reference to the struct instance( no ownership moved, have permission to mutate)
        self.duration_sec *= 2;
        println!("duration_sec: {:#?}", self);
    }
}
#[test]
fn defining_struct_methods(){
    let song = TaylorSwiftSong{
        title: String::from("Love Story"),
        release_year: 2008,
        duration_sec: 235,
    };
    song.display_song_info();
}
#[test]
fn self_parameter_as_mutable_struct_instance(){
    let song = TaylorSwiftSong{
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
fn self_parameter_as_immutable_reference_and_mutable_reference_to_struct_instance(){
    let mut song = TaylorSwiftSong{
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