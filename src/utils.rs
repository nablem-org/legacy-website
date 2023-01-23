use std::collections::HashMap;
use std::process::exit;
use std::error::Error;
use std::fmt::Write;
use std::{fs, io};
use core::str::Split;
use strfmt::strfmt;

/// A three-dimensional hash map that stores category data of each work, parsed from a text file
pub type Hash = HashMap<String, HashMap<String, Vec<String>>>;

/// A multi-dimensional heap-allocated array used to allocate its hash map counterpart
pub type HashLike = Vec<(String, Vec<(String, Vec<String>)>)>;

/// An encapsulation type for all propagated standard errors; taken from
/// ["`?` couldn't convert the error to `std::io::Error`"](https://stackoverflow.com/a/62273886)
pub type BoxError = Result<(), Box<dyn Error>>;

/// A pointer to a mutable hash map designed for [`strfmt`](https://github.com/vitiral/strfmt)
type FmtHash<'a> = &'a mut HashMap<String, String>;


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

/// Inserts a key from a string slice
fn insertKey(h: FmtHash, key: &str, value: String) {
    h.insert(key.to_string(), value);
}

/// Inserts the keys (and their values) required to format the template into a hash map,
/// where the inner key is set again on the template string to be formatted later;
/// then, formats the HTML template with the map.
fn insertIntoMapAndFormat(h: FmtHash, l: String, d: &str) -> String {
    insertKey(h, "h", l.clone());
    insertKey(h, "id", l.to_lowercase());
    insertKey(h, "inner", "{inner}".to_string());
    strfmt(d, &h).unwrap()
}

/// Inserts the value to the inner key and formats the HTML template
fn insertInner(h: FmtHash, s: String, d: String) -> (String, String) {
    h.insert("inner".to_string(), d.clone());
    (strfmt(&s, &h).unwrap(), String::new())
}

/// Indents a multi-line string, N times, by splitting the string on each new line,
/// and prepending each line with tab characters.
fn tabMultiline(s: String, n: usize, tab: bool) -> String {
    let mut tabbed = String::new();

    let mut i: i8 = 0;
    for l in s.split("\n") {
        // Prepends each line with a tab efficiently, other than the first one
        let mut tabbedLine = l.to_string();

        if i != 0 || tab {
            let tabs = "\t".repeat(n);
            tabbedLine.insert_str(0, &tabs);
        }

        // Adds each line into the new variable
        write!(&mut tabbed, "{}\n", tabbedLine).unwrap(); i += 1;
    }

    tabbed
}

/// Formats the HTML template (e.g. genre or item) from its file, indents the code accordingly,
/// and stores the result on the element variable
fn formatMultiline(mut h: FmtHash, e: &mut String, d: &str, l: String,  n: usize, tab: bool) {
    let f = insertIntoMapAndFormat(&mut h, l, d);
    let tabbed = tabMultiline(f, n, tab);
    write!(e, "{tabbed}").unwrap();
}

/// Returns false if the accumulator is empty, which means will not tab all lines
fn tab(acc: &str) -> bool {
    match acc {
        "" => false,
        _ => true
    }
}

/// Splits an item statement (containing a banner path definition, separated with a slash) in its title and banner
/// in the form of a wrapper function, where two functions are passed: one if the statement contains a separator (i.e. true)
/// and another if not (i.e. false)
fn lineDivide<Ft, Ff>(i: String, ft: Ft, ff: Ff) -> String
    where Ft: FnOnce(Split<&str>) -> String,
          Ff: FnOnce(String) -> String {
    let sep = " - ";
    match i.contains(sep) {
        true => ft(i.split(sep)),
        false => ff(i)
    }
}

/// Always retrieves the title of an item statement without its banner path, if found
fn title(i: String) -> String {
    lineDivide(i, |mut s| s.next().unwrap().to_string(), |s| s)
}

/// Retrieves the banner path instead of the title, out of an item statement, if found; otherwise,
/// returns a default value
fn bannerPath(i: String, default: &str) -> String {
    let base = "../images";
    let defaultPath = format!("{base}/{default}");
    lineDivide(i, |s| format!("{base}/{}", s.last().unwrap()).to_string(), |_| defaultPath.to_string())
}

/// Allocates a multi-dimensional hash map, i.e. a collection of key-value pairs [<sup>1</sup>][1], that stores data
/// from the categories each item is contained from a plain-text file. Additionally generates the HTML code
/// for each item (i.e. the UI) into a template file, due to the use of a match statement on the indentation count.
///
/// ## HTML
///
/// Based on the indentation count, the HTML is formatted from individual templates for each type of category
/// (e.g. genres and items) in the form of a hierarchy, of which the title (and thence its ID) are substituted therein,
/// including the image path for items.
///
/// These templates can be found in `templates/ui`.
///
/// Outside the loop, the code of each type of category, their accumulators and indexes are defined.
/// Accumulators build up the formatted code of said type, i.e. from items to genres and then to sections;
/// whereas indexes avoid an unnecessary write to an accumulator in its first iteration.
/// Both of these apply to genres and items, but not sections, which just add to the overall code.
///
/// Additionally, the name of the last section is kept to determine the "Coming Soon" image to be used,
/// as the default path, if no image was defined.
///
/// *The code logic may be messy, but it can be put as such.*
///
/// [1]: https://doc.rust-lang.org/book/ch08-03-hash-maps.html
pub fn makeHashMapAndUI() -> Hash {
    let mut h = HashMap::new();
    let mut html = String::new();

    let base = "templates/ui";
    let section = fs::read_to_string(format!("{base}/section.html")).unwrap(); let mut s = String::new(); let mut s_l = String::new();
    let genre = fs::read_to_string(format!("{base}/genre.html")).unwrap(); let mut g = String::new(); let mut g_acc = String::new(); let mut g_i: i8 = 0;
    let item = fs::read_to_string(format!("{base}/item.html")).unwrap(); let mut i = String::new(); let mut i_acc = String::new(); let mut i_i: i8 = 0;

    // Reads the plain text file and splits into lines
    let raw = fs::read_to_string("data").unwrap();
    let lines = raw.split("\n");

    // Instantiates a new Vec to push each value into the array to convert them into key-value pairs
    let mut v: HashLike = Vec::new();

    for line in lines {
        if line != "" {
            // Checks the amount of indentation (i.e. tabs) of each line to know what to push;
            // e.g. if a category, genre or title
            let l = line.trim().to_string();
            match line.chars().filter(|l| l.is_control()).count() {
                0 => {
                    s_l = l.clone();

                    // Writes the last genre and item not catched within their statements to their accumulators;
                    // whereas, in the genre, the accumulated code is inserted
                    write!(&mut i_acc, "{i}").unwrap(); i = String::new();
                    (g, i_acc) = insertInner(&mut h, g, i_acc);
                    write!(&mut g_acc, "{g}").unwrap();

                    // Inserts the accumulated code from all the genres inside the section element,
                    // and resets the accumulator and index to zero
                    (s, g_acc) = insertInner(&mut h, s, g_acc); g_i = 0;

                    // Appends the section variable code that is to be fully formatted to the overall code,
                    // which means its value should be empty on the first iteration;
                    // and then, thus, empties its value when its defined
                    write!(&mut html, "{s}").unwrap();
                    s = String::new();

                    // Substitutes the values (i.e. the ID and header names) from the section HTML template
                    // and writes the formatted code into its corresponding variable
                    let f = insertIntoMapAndFormat(&mut h, l.clone(), &section);
                    write!(&mut s, "{f}").unwrap();


                    // Pushes a new category with an empty genre array
                    v.push((l, Vec::new()));
                },
                1 => {
                    // If not on the first iteration, writes the last item to its accumulator; inserts
                    // the code inside the genre element, and accumulates the result to then reset values
                    if g_i != 0 {
                        write!(&mut i_acc, "{i}").unwrap();
                        (g, i_acc) = insertInner(&mut h, g, i_acc); i_i = 0; i = String::new();
                        write!(&mut g_acc, "{g}").unwrap();
                    }; g = String::new(); g_i += 1;

                    // Formats its HTML template, indents the code and stores the result into its variable
                    formatMultiline(&mut h, &mut g, &genre, title(l.clone()), 1, tab(&g_acc));


                    // Pushes a new genre with an empty title array into the last category
                    let (_, category) = v.last_mut().unwrap();
                    category.push((l.clone(), Vec::new()));
                },
                2 => {
                    // If not id. writes to accumulator
                    if i_i != 0 {
                        write!(&mut i_acc, "{i}").unwrap();
                    }; i_i += 1; i = String::new();

                    // Formats the template with both the title and the banner path as the image source;
                    // whereas the default value is the path to its corresponding "Coming Soon" image from the
                    // section name
                    let defaultPath = match s_l.as_str() {
                        "Shows" => "shows-soon.png",
                        "Anime" => "shows-soon.png",
                        "Manga" => "anime-soon.png",
                        "Games" => "games-soon.png",
                        _ => ""
                    };

                    let title = title(l.clone());
                    let bannerPath = bannerPath(l.clone(), defaultPath);

                    insertKey(&mut h, "img", bannerPath);
                    formatMultiline(&mut h, &mut i, &item, title.clone(), 2, tab(&i_acc));

                    // Pushes a new title into the last genre list
                    let (_, category) = v.last_mut().unwrap();
                    let (_, genre) = category.last_mut().unwrap();

                    genre.push(title);

                },
                _ => panic!("Bad")
            }
        }
    }

    println!("{html}");

    // Writes the formatted layout code into a file
    match fs::write("templates/ui.html", html) {
        Ok(_) => println!("Written to UI HTML file successfully!"),
        Err(e) => eprintln!("{e}")
    }

    // Allocates the multi-dimensional Vec into a hash map, i.e. key-value pairs [1-3]
    let mut m: Hash = HashMap::new();

    for (key, genres) in v {
        let mut g = HashMap::new();
        for (genre, titles) in genres {
            g.entry(genre).or_insert(titles);
        }
        m.entry(key).or_insert(g);
    }

    m
}

/*
    [1]: https://doc.rust-lang.org/book/ch08-03-hash-maps.html
    [2]: https://stackoverflow.com/a/30441736
    [3]: https://doc.rust-lang.org/std/collections/struct.HashMap.html#examples
    [4]: https://stackoverflow.com/a/62273886
*/
