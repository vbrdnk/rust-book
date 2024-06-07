use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let blue_team_score = scores.get("Blue");
    let red_team_score = scores.get("Red");
    match red_team_score {
        Some(score) => println!("Red team score: {}", score),
        None => println!("Red team score: 0"),
    }
    println!("Blue team score: {}", blue_team_score.unwrap());

    let blue_team_score = scores.get("Blue").copied().unwrap_or(0);
    let red_team_score = scores.get("Red").copied().unwrap_or(0);
    println!("Red team score: {}", red_team_score);
    println!("Blue team score: {}", blue_team_score);

    // iterate over the map
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // adding a key and value only if it doesn't exist
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // update the value base on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
