use std::collections::HashMap;

fn main() {
// Creating a Hash Map
	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);

	let teams = vec![String::from("Blue"), String::from("Yellow")];
	let initial_scores = vec![10, 50];

	let mut scores: HashMap<_, _> = 
		teams.into_iter().zip(initial_scores.into_iter()).colletc();

// Hash Maps and ownership
	let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point

// Accessing Values in a Hash map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
    	println!("{}: {}", key, value);
    }

// Updating a Hash Map
	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Blue"), 25);

	println!("{:?}", scores);

	// Only inserting a value if the key has no value
	let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    //Updating a value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
