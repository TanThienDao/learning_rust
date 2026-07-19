//use super::lodging::{Accommodation, Description};
use crate::lodging::{Accommodation, Description};

pub fn book_for_one_night<T: Accommodation + Description>(entity: &mut T, guest: &str) {
    println!(
        "Booking for one night at {} for {}",
        entity.get_description(),
        guest
    );
    entity.book(guest, 1);
}

pub fn mix_and_match<T, G>(first: &mut T, secound: &mut G, guest: &str)
where
    T: Accommodation + Description,
    G: Accommodation,
{
    first.book(guest, 1);
    first.get_description();
    secound.book(guest, 1);
}
