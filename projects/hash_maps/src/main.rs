use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Combining 2 vectors into a HashMap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores);

    // Accessing values in a HashMap
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    if let Some(score) = scores.get(&team_name) {
        println!("Team name: {}, Score: {}", team_name, score);
    }

    // Iterate over each key/value pair with a for loop
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // this println! will only work if `scores` is a reference in for loop above (line 24)
    println!("{:?}", scores);

    // Only inserting a value if the key has no value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // dereference `count` to update the actual value in heap memory
        *count += 1;
    }

    println!("{:?}", map);
}
