struct Location {
    name: String,
    treasures: u32,
}

struct Map<'a> {
    locations: &'a [Location],
}

impl<'a> Map<'a> {
    fn explore<F>(&self, mut action: F)
    where
        F: FnMut(&'a Location),
    {
        let final_indedx = self.locations.len() - 1;
        let mut current_index = 0;
        while current_index <= final_indedx {
            let current_location = &self.locations[current_index];
            action(current_location);
            current_index += 1;
        }
    }
}
fn main() {
    let location = [
        Location {
            name: String::from("Enchanted Forest"),
            treasures: 5,
        },
        Location {
            name: String::from("Mystic Mountain"),
            treasures: 10,
        },
        Location {
            name: String::from("Hidden Valley"),
            treasures: 7,
        },
        Location {
            name: String::from("Crystal Lake"),
            treasures: 2,
        },
    ];
    let map = Map {
        locations: &location[0..3],
    };

    let mut total_treasure = 0;
    let mut location_vec: Vec<String> = Vec::new();

    map.explore(|location| {
        total_treasure += location.treasures;
    });

    println!("{:?}", total_treasure);

    map.explore(|location| {
        location_vec.push(location.name.clone());
    });
    println!("{:?}", location_vec);

    let mut location_vec_borrow: Vec<&str> = Vec::new();
    map.explore(|location| {
        location_vec_borrow.push(&location.name);
    });
    println!("{:?}", location_vec_borrow);
}
