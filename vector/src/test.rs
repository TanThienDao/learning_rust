#[test]
fn create_a_vector(){
    // this is an array
    let rick_moranis_movies = ["Ghostbuster","Honey I Shrunk the Kids","Space-balls"];
    //  Vector
    // do not implement display trail
    // but implement debug  trail out of the box.
    let pizza_diameters:Vec<i32> = Vec::new();
    let pizza_diameters_2 = Vec::<i32>::new();
    println!("{:?}", rick_moranis_movies);
    println!("{:?} {:?}", pizza_diameters, pizza_diameters_2);

    let pastas:Vec<&str> = Vec::new();

    let pizza_diameters_3 = vec![12, 16, 24];
    println!("{:#?}", pizza_diameters_3);

}
#[test]
fn adding_and_removing_elements(){
    let mut pizza_diameters = vec![8,10,12,14];
    pizza_diameters.push(16);
    pizza_diameters.push(18);
     println!("{:#?}", pizza_diameters);
    pizza_diameters.insert(2, 11);
    println!("{:#?}", pizza_diameters);
    let last_pizza_diameter =pizza_diameters.pop();
    println!("{:#?}", last_pizza_diameter);
    println!("{:?}", pizza_diameters);
    let third_diameter_from_start = pizza_diameters.remove(2);
    println!("{:?}", third_diameter_from_start);
    println!("{:#?}", pizza_diameters);
}
#[test]
fn reading_vector_elements(){
    let pizza_diameters = vec![8,10,12,14];
    println!("{:#?}", pizza_diameters);
    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    // ownership is move from those var to the vector
    let pizza_topping = vec![pepperoni, mushroom, sausage];

    let value = pizza_diameters[2];
    //create a copy of the pizza diameter to the value !
    //pizza diameter still the owner of that value.
    println!("{:#?}", value);

    //compiler will not happy because this is a heap allocated string
    // it does not implement the copy trait
    // this is force us to move the ownership of that string from vector to value
    // and that now allow by compiler
    //let value_2 = pizza_topping[2];
    let reference = &pizza_diameters[2];
    println!("{:#?}", reference);

    // this will cause an error at runtime because compiler
    // can not determine or know the size of the growable vector.
    //let invalid_value = &pizza_diameters[200];
    //println!("{:#?}", invalid_value);

    let pizza_slide = &pizza_diameters[1..3];
    println!("{:#?}", pizza_slide);
}
#[test]
fn the_get_method(){
    // The get method extracts a vector element by index position,
    // It returns an Option enum.
    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let pizza_topping = vec![pepperoni, mushroom, sausage];

    let option = pizza_topping.get(50);
    println!("{:#?}", option);

    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let mut pizza_topping = vec![pepperoni, mushroom, sausage];

    pizza_topping[1] = String::from("Olives");
    println!("pizza topping {:#?}", pizza_topping);

    let target_topping = &mut pizza_topping[2];
    //let another_topping = &mut pizza_topping[1];
    target_topping.push_str("and Meatball");
    let another_topping = &mut pizza_topping[2];
    println!("pizza topping {:#?}", pizza_topping);
}
#[test]
fn vector_capacity_behind_the_scenes(){
    // The vector capacity is the maximum number of elements that
    // the vector can contain.
    let mut seasons:Vec<&str>=  Vec::with_capacity(4);
    println!("Length: {}. Capacity: {}",
             seasons.len(),
             seasons.capacity()
    );
    seasons.push("Summer");
    seasons.push("Fall");
    seasons.push("Winter");
    seasons.push("Spring");
    println!("Length: {}. Capacity: {}",
             seasons.len(),
             seasons.capacity()
    );
    let test = &mut seasons[3];
    seasons.push("Ring");
    println!("Length: {}. Capacity: {}",
             seasons.len(),
             seasons.capacity()
    );
    //println!("test: {:#?}", test);
}
#[derive(Debug)]
struct File{
    name: String,
}
#[derive(Debug)]
struct Folder{
    name: String,
    contents: Vec<File>,
}
impl Folder{
    fn new(name: String) -> Self{
        Self{
            name: name,
            contents: Vec::new(),
            //contents: ven![]
        }
    }

    fn create_file(&mut self, name: String) {
        self.contents.push(File{name: name});
    }
    fn delete_file(&mut self, index: usize) ->File{
         self.contents.remove(index)
    }
    fn get_file(&mut self, index: usize) -> Option<&File> {
        self.contents.get(index)

    }
}
#[test]
fn project_section_13_vector(){
    let mut folder = Folder::new("Practice".to_string());
    folder.create_file("File practice".to_string());
    folder.create_file("Disk practice".to_string());
    println!("Practice folder: {:#?}", folder);
    folder.delete_file(0);
    println!("File removed: {:#?}", folder);
    let check = folder.get_file(0);
    match check {
        Some(file) => println!("File found: {:#?}", file),
        None => println!("There was no file"),
    }
}