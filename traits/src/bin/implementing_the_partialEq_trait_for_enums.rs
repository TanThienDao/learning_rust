//#[derive(PartialEq)]
enum Musician {
    SingleSingWriter(String),
    Band(u32),
}
use Musician::{Band, SingleSingWriter};

impl PartialEq for Musician {
    fn eq(&self, other: &Self) -> bool {
        match self {
            SingleSingWriter(name) => match other {
                SingleSingWriter(other_name) => name == other_name,
                Band(_) => false,
            },
            Band(members) => match other {
                Band(other_members) => members == other_members,
                SingleSingWriter(_) => false,
            },
        }
    }
}
fn main() {
    let single_writer = Musician::SingleSingWriter(String::from("Holly"));
    let rustin_timberl = Musician::SingleSingWriter(String::from("Rustin Timbler"));
    let holly = Musician::SingleSingWriter(String::from("Holly"));
    let band = Musician::Band(4);
    let band_noone = Musician::Band(5);
    let rust_for_revengent = Musician::Band(5);

    println!("{:?}", single_writer.eq(&rustin_timberl));
    println!("{:?}", single_writer.eq(&holly));

    println!("{:?}", single_writer.eq(&band));

    println!("{:?}", band_noone.eq(&band));
    println!("{:?}", band_noone.eq(&rust_for_revengent));
}
