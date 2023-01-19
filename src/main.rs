#![allow(non_snake_case)]

mod utils;
use utils::*;

use askama::Template;
use std::collections::HashMap;
use std::fs;

#[derive(Template)]
#[template(path = "index.html")]
struct Index {}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutUs {}

#[derive(Template)]
#[template(path = "subscription.html")]
struct Subscription {}

#[derive(Template)]
#[template(path = "games.html")]
struct Games {}

/// Renders each HTML template into HTML files fit for deployment
fn render() -> BoxError {
    // If the HTML path does not exist, creates the directory
    if !exists("html") {
        fs::create_dir("html")?;
        println!("> Created HTML directory!");
    } else {
        // If the HTML path already exists, checks if it does not point to a file
        if file("html") {
            error(">>> html is a file!");
        }
    }

    // Renders each template into HTML, and writes the output into a file
    writeToFile(Index {}.render()?, "index")?;
    writeToFile(AboutUs {}.render()?, "about")?;
    writeToFile(Subscription {}.render()?, "subscription")?;
    writeToFile(Games {}.render()?, "games")?;

    // Propagates any error
    Ok(())
}

fn makeHashMap() -> Hash {
    let raw = fs::read_to_string("data").unwrap();
    let lines = raw.split("\n");

    let mut v: HashLike = Vec::new();

    for line in lines {
        if line != "" {
            let l = line.trim().to_string();
            match line.chars().filter(|l| l.is_control()).count() {
                0 => {
                    // Pushes a new category with a list of genres
                    v.push((l, Vec::new()))
                },
                1 => {
                    // Pushes a new genre with a list of titles into the last category
                    let (_, category) = v.last_mut().unwrap();
                    category.push((l, Vec::new()));
                },
                2 => {
                    // Retrieves the title without its banner from the line
                    let s = " - ";
                    let title = match l.contains(s) {
                        true => l.split(s).next().unwrap().to_string(),
                        false => l
                    };

                    // Pushes a new title into the last genre list
                    let (_, category) = v.last_mut().unwrap();
                    let (_, genre) = category.last_mut().unwrap();

                    genre.push(title);
                },
                _ => panic!("Bad")
            }
        }
    }

    // Dynamically allocates the tuple vector into a hash map, i.e. key-value pairs [1 & 2], including the title lists
    let mut m: Hash = HashMap::new();

    for (key, genres) in v {
        let mut g = HashMap::new();
        for (genre, titles) in genres {
            g.entry(genre).or_insert(titles);
        }
        m.entry(key).or_insert(g);
    }

    bracketizer(m)
}

fn main() {
    let map = makeHashMap();
    println!("{:?}", map);
    hashLoop(&map, |category, genre, title| println!("{title} -> {genre} -> {category}"));
    render().unwrap();
}

/*
    [1]: https://doc.rust-lang.org/book/ch08-03-hash-maps.html
    [2]: https://stackoverflow.com/a/30441736
    [3]: https://doc.rust-lang.org/std/collections/struct.HashMap.html#examples
    [4]: https://doc.rust-lang.org/std/collections/hash_map/struct.OccupiedEntry.html#method.get
*/
