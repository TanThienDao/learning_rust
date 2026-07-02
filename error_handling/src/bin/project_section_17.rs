fn main() {
    let file_result = write_to_file();
    match file_result {
        Ok(file) => {
            println!("File written to: {}", file)
        }
        Err(error) => {
            eprintln!("Error writing to file: {:#?}", error);
            std::process::exit(1);
        }
    }
}
fn write_to_file() -> Result<String, std::io::Error> {
    println!("What file would you like to write to?");
    let mut file = String::new();
    std::io::stdin().read_line(&mut file)?;
    println!("What would you like to write to the file?");
    let mut contents = String::new();
    std::io::stdin().read_line(&mut contents)?;
    let path = std::path::Path::new(file.trim());
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(&parent)?;
        }
    }
    std::fs::write(&path, &mut contents)?;
    Ok(file.to_string())
}
