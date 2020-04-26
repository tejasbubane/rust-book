use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);
    println!("scores = {:?}", scores);

    let teams = vec!["Blue", "Yellow"];
    let init_scores = vec![10, 40];

    let scores: HashMap<_, _> =
        teams.into_iter().zip(init_scores.into_iter()).collect();
    println!("scores = {:?}", scores);

    let name = String::from("Color");
    let value = String::from("blue");
    let mut map = HashMap::new();
    map.insert(name, value);
    // name & value are `moved` into map & are no longer accessible

    // Accessing values in HashMap
    // get returns Option - `Some(&value)` if found otherwise `None`
    println!("Blue = {:?}", scores.get("Blue"));

    // Iterate over HashMap

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // overwriting value - just insert
    println!("Before map = {:?}", map);
    map.insert(String::from("Color"), String::from("red"));
    println!("After map = {:?}", map);

    // only insert if key not present - entry.or_insert
    map.entry(String::from("Country")).or_insert(String::from("India"));
    map.entry(String::from("Color")).or_insert(String::from("black"));
    println!("Updated map = {:?}", map);

    // Update based on old value
    // entry.or_insert returns mutable reference - use it to update value
}
