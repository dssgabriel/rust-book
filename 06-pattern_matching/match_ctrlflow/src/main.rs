#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

// Matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x { // Exhaustive, every possibility must be covered
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
	let five = Some(5);
	let six = plus_one(five);
	let none = plus_one(None);

	// _ placeholder
	let some_u8_value = 0u8;
	match some_u8_value {
	    1 => println!("one"),
	    3 => println!("three"),
	    5 => println!("five"),
	    7 => println!("seven"),
	    _ => (),
	}

	let some_u8_value = Some(0u8);
	// Equivalent to this match expression 
	match some_u8_value {
	    Some(3) => println!("three"),
	    _ => (),
	}
	// if let syntax makes less verbose to handle a single pattern
	// and igore the rest
	if let Some(3) = some_u8_value {
    	println!("three");
	}

	// Same example
	let mut count = 0;
	let coin = Coin::Dime;
	match coin {
    	Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    	_ => count += 1,
	}

	// if let & else
	let mut count = 0;
	let coin = Coin::Penny;
	if let Coin::Quarter(state) = coin {
	    println!("State quarter from {:?}!", state);
	} else {
	    count += 1;
	}
}
