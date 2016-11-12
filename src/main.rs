mod read;

mod search;
mod search_test;

fn main() {
    let filename = std::env::args().nth(1).unwrap_or(String::from(""));
    let pattern = std::env::args().nth(2).unwrap_or(String::from(""));

    match read::read(&filename) {
        Err(e) => println!("Failed to read {}: {}", filename, e),
        Ok(content) => {
            match search::search(&content, &pattern.as_bytes()) {
                Some(line) => println!("{}", String::from_utf8_lossy(line)),
                None => println!("No match found."),
            }
        }
    }
}
