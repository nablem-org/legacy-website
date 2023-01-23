use std::collections::hash_map::Entry;

fn hashLoop<'a, F>(m: &Hash<'a>, f: F)
    where F: Fn(&str, &'a str, &str, &mut Hash) {
    let mut n = m.clone();
    for (category, genres) in m {
        for (genre, titles) in genres {
            for title in titles {
                f(category, genre, title, &mut n);
            }
        }
    }
}

fn bracketizer<'a>(m: Hash<'a>) -> Hash<'a>  {
    hashLoop(&m, |category, genre, title, n| {
        //
    }); m
}

/// Checks if a line contains a category that finds to be enclosed by brackets,
/// the category's genre items are imported therein, printing the genre and category (futile!).
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

fn writeGenre(
    mut g: String,
    mut i_acc: String,
    g_acc: &mut String,
    i: &mut String,
    mut h: FmtHash,
) -> (String, String, i8) {
    write!(&mut i_acc, "{i}").unwrap();
    (g, i_acc) = insertInner(&mut h, g, i_acc);
    write!(g_acc, "{g}").unwrap();
    (g, i_acc, 0)
}



// [4]: https://doc.rust-lang.org/std/collections/hash_map/struct.OccupiedEntry.html#method.get
