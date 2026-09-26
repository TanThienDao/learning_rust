//! The fs::read_dir() function returns a io::Result<ReadDir> enum.
//! The ReadDir struct implements the Iterator trait.
//! The iterator yields Result<DirEntry, Error> enums.
//! THe DirEntry struct supports a 'path' method.
//! the fs::metadata() function reutnr a Metadata struct.
//! the Metadata struct includes an 'is_file' method.
//! the fs::read_to_string() function returns a io::Result<String>.
//!
use std::fs;
use std::io;
/*fn main() -> io::Result<()> {
    for entry in fs::read_dir("./")? {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                let metadata = fs::metadata(&path).unwrap_or_else(|e| {
                    eprintln!("Error getting metadata for {:?}: {}", path, e);
                    std::process::exit(1);
                });
                if metadata.is_file() {
                    let content = fs::read_to_string(&path).unwrap_or_else(|e| {
                        eprintln!("Error reading file {:?}: {}", path, e);
                        std::process::exit(1);
                    });
                    println!("File: {:?}, Content: {}", path, content);
                } else {
                    println!("Directory: {:?}", path);
                }
            }
            Err(e) => {
                eprintln!("Error reading entry: {}", e);
                std::process::exit(1);
            }
        }
    }
    Ok(())
}*/
fn main() -> io::Result<()> {
    for entry_result in fs::read_dir("./")? {
        let entry = entry_result?;
        println!("Entry: {:?}", entry.path());
        let metadata = fs::metadata(entry.path())?;
        if metadata.is_file() {
            let content = fs::read_to_string(entry.path())?;
            println!(
                "File: {:?}, Content: \n---------\n{}\n========",
                entry.path(),
                content
            );
        } else {
            println!("Directory: {:?}", entry.path());
        }
    }

    Ok(())
}
