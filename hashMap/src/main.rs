
use std::collections::HashMap;

fn main() {

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // creacion the un hash a partir de dos vectores pasados a una tupla y
    // esta convertida a un hash utilizando collect
    let teams  = vec![String::from("co"), String::from("gfr")];
    let initial_scores = vec![1000, 500];

    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    //ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    //print!("{}", field_name); //error
    println!("{:?}", map);
    println!("get element from scores {:?}", &scores.get("Blue"));
    println!("get element from scores2 {:?}", &map.get("Yellow"));

    for (key, value) in &scores2 {
        println!("{}: {}", key, value);
    }

    //un key puede conteneer un solo valor a la vez
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores); // "Blue":25

    // Using the entry method to only insert if the key does not already have a value
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
    //otra utilidad de entry, contar el numero de veces que una palabra se usa o etc..
    let text = "hello world wonderful world beautiful world and wonderful place for be in this place every day of my place in this world yeah!!!";
    let mut map2 = HashMap::new();

    for word in text.split_whitespace() {
        let count = map2.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map2);
        /*
        By default, HashMap uses a cryptographically secure hashing function that can provide resistance to Denial of Service (DoS) attacks.
        This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it.
        If you profile your code and find that the default hash function is too slow for your purposes, you can switch to another function by specifying a different hasher.
        A hasher is a type that implements the BuildHasher trait. We’ll talk about traits and how to implement them in Chapter 10.
        You don’t necessarily have to implement your own hasher from scratch; crates.io has libraries shared by other Rust users that provide
        hashers implementing many common hashing algorithms.
        */

}
