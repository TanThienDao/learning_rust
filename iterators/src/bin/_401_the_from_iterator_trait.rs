//! from_iter

use std::collections::HashSet;
#[derive(Debug)]
struct Playlist {
    songs: Vec<String>,
    users: HashSet<String>,
}
impl FromIterator<(String, String)> for Playlist {
    fn from_iter<I: IntoIterator<Item = (String, String)>>(iter: I) -> Self {
        let mut songs = Vec::new();
        let mut users = HashSet::new();
        for (song, user) in iter {
            songs.push(song.clone());
            users.insert(user.clone());
        }
        Playlist { songs, users }
    }
}
fn main() {
    /*    let fifty = 1..=50;
    let fifty_vec = Vec::from_iter(fifty.clone());
    println!("{:?}", fifty_vec);
    let results_2 = fifty.clone().collect::<Vec<i32>>();
    println!("{:?}", results_2);

    let unique_set: HashSet<i32> = HashSet::from_iter(fifty_vec);
    println!("{:?}", unique_set);

    let unique_set_2: HashSet<i32> = fifty.clone().collect::<HashSet<i32>>();
    println!("{:?}", unique_set_2);

    let char = ['H', 'e', 'l', 'l', 'o'];
    let char_vec = Vec::from_iter(char.clone());
    println!("{:?}", char_vec);
    let greeetin= String::from_iter(char.clone());
    println!("{:?}", greeetin);*/

    let song = [
        (String::from("Song 1"), String::from("User A")),
        (String::from("Song 2"), String::from("User B")),
        (String::from("Song 3"), String::from("User A")),
        (String::from("Song 4"), String::from("User C")),
    ];

    let playlist = Playlist::from_iter(song.clone());
    println!("{:?}", playlist);

    let playlist2 = song.into_iter().collect::<Playlist>();
    println!("{:?}", playlist2);
}
