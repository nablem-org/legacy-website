use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::process::exit;
use std::{fs, io};

/// Checks if a path points to a file
pub fn exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

/// Checks if a path does not point to a directory
pub fn file(path: &str) -> bool {
    fs::metadata(path).expect("{path} does not exist").is_file()
}

/// Prints to standard error and exits the process without panicking
pub fn error(e: &str) {
    eprintln!("{e}"); exit(0);
}

/// Writes HTML into a file, and prints the event
pub fn writeToFile(html: String, name: &str) -> io::Result<()> {
    fs::write(format!("html/{}.html", name), html).unwrap();
    println!("> Written {} HTML into a file!", name);
    Ok(())
}

pub type HashLike = Vec<(String, Vec<(String, Vec<String>)>)>;
pub type Hash = HashMap<String, HashMap<String, Vec<String>>>;
pub type BoxError = Result<(), Box<dyn std::error::Error>>;

/// Loops over a multi-dimensional hash map (i.e. dictionary-like)
pub fn hashLoop<F>(m: &Hash, f: F)
    where F: Fn(&str, &str, &str) {
    for (category, genres) in m {
        for (genre, titles) in genres {
            for title in titles {
                f(category, genre, title);
            }
        }
    }
}

/// Checks if a line contains a category that finds to be enclosed by brackets,
/// the category's genre items are imported therein, printing the genre and category
pub fn bracketizer(m: Hash) -> Hash {
    let mut n = m.clone();
    for (category, genres) in &m {
        for (genre, titles) in genres {
            for title in titles {
                if title == "[Manga]" {
                    println!("{genre} in {category}");
                    if let Some(o) = m.get("Manga") {
                        if let Some(x) = o.get(genre.as_str()) {

                            // Seeks the key via a clone of the map, and removes it
                            // in order to replace its value by setting it on a new key;
                            // alternatively prints the orignal value

                            let c = n.get_mut(category.as_str()).unwrap();
                            let pv = c.remove_entry(genre.as_str()).unwrap();
                            c.entry(genre.to_string()).or_insert(x.to_vec());
                            println!("> {:?}", pv);

                            // Checks that key has been modified properly by printing its value [4]
                            if let Entry::Occupied(e) = c.entry(genre.to_string()) {
                                println!("< {:?}", e.get())
                            }
                        }
                    }
                }
            }
        }
    }
    n
}
