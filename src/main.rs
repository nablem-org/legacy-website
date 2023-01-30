#![allow(non_snake_case)]

mod utils;
use utils::*;

use askama::Template;
use std::fs;

#[derive(Template)]
#[template(path = "index.html")]
struct Index {}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutUs {}

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
    writeToFile(Games {}.render()?, "games")?;

    // Propagates any error
    Ok(())
}

fn main() {
    let map = makeHashMapAndUI();
    hashLoop(&map, |category, genre, title|
        println!("{title} -> {genre} -> {category}"));
    render().unwrap();
}

