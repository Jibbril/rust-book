use std::collections::HashMap;

pub fn main() {
    println!("================ main called from hashmaps! ================");
    init_hashmap();
    iter_init_hashmap();
    get_from_hashmap();
    iterate_over_hashmap();
    overwrite_hashmap_value();
    update_if_empty();
    update_based_on_previous();
}

fn init_hashmap() {
    let mut scores = HashMap::new();

    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 50);
}

fn iter_init_hashmap() {
    let teams = vec!["Blue".to_string(), "Yellow".to_string()];
    let initial_scores = vec![10,50];

    let mut _scores: HashMap<_,_> = 
        teams.into_iter().zip(initial_scores.into_iter()).collect();
}

fn get_from_hashmap() {
    println!("--------------- get from hashmap ---------------");
    let mut scores = HashMap::new();
    let blue = "Blue";

    scores.insert(blue.to_string(), 10);
    scores.insert("Yellow".to_string(), 50);

    let score = scores.get(blue);
    if let Some(n) = score {
        println!("Blue's score is: {}",n);
    }
}

fn iterate_over_hashmap() {
    println!("--------------- iterate over hashmap ---------------");
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn overwrite_hashmap_value() {
    println!("--------------- overwrite hashmap value ---------------");
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
}

fn update_if_empty() {
    println!("--------------- update if empty ---------------");
    let mut scores = HashMap::new();

    let blue = "Blue".to_string();
    let yellow = "Yellow".to_string();

    scores.insert(&blue,10);
    scores.entry(&blue).or_insert(50); // Won't update since blue has value
    scores.entry(&yellow).or_insert(50); // Will update value to 50

    println!("{:?}", scores);
}   

fn update_based_on_previous() {
    println!("--------------- update based on previous ---------------");
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

