struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn main() {
    let x = Some(5);
    let y = 10;

    // Starts new scope, will match on second arm
    match x {
        Some(50) => println!("Got 50!"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("At the end: x = {:?}, y = {:?}", x, y);

    // Multiple patterns with `|`
    let x = 1;

    match x {
        1 | 2 => println!("One or two"),
        3 => println!("Three"),
        _ => println!("Anything!"),
    }

    // Matching ranges with `..=`
    match x {
        1..=5 => println!("One through five"),
        _ => println!("Something else"),
    }

    // Works with ASCII characters too...
    let y = 'c';

    match y {
        'a'..='j' => println!("Early ASCII letter"),
        'k'..='z' => println!("Late ASCII letter"),
        _ => println!("Something else"),
    }

    // Destructuring structs
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    println!("a = {}, b = {}", a, b);

    // Shortand notation
    let Point { x, y } = p;
    println!("x = {}, y = {}", x, y);

    // Another example
    match p {
        Point { x: 0, y: 0 } => println!("At the origin!"),
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis! ({}, {})", x, y),
    }

    // Destructuring enums
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        },
        Message::Move { x, y } => {
            println!(
                "Move by {} in the x direction and by {} in the y direction",
                x, y
            );
        },
        Message::Write(text) => {
            println!("Text message: {}", text);
        },
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {} and blue {}",
                r, g, b
            );
        },
    }
}
