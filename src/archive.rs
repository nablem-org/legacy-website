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

// If a line contains a category found to be enclosed by brackets,
// the category's genre items are imported therein, printing the genre and category
fn bracketizer<'a>(m: Hash<'a>) -> Hash<'a>  {
    hashLoop(&m, |category, genre, title, n| {
        if title == "[Manga]" {
            println!("{genre} in {category}");
            if let Some(o) = m.get("Manga") {
                if let Some(x) = o.get(genre) {

                    // Seeks the key via a clone of the map, and removes it
                    // in order to replace its value by setting it on a new key;
                    // alternatively prints the orignal value

                    let c = n.get_mut(category).unwrap();
                    let pv = c.remove_entry(genre).unwrap();
                    c.entry(genre).or_insert(x.to_vec());
                    println!("> {:?}", pv);

                    // Checks that key has been modified properly by printing its value [4]
                    if let Entry::Occupied(e) = c.entry(genre) {
                        println!("< {:?}", e.get())
                    }
                }
            }
        }
    }); m
}
