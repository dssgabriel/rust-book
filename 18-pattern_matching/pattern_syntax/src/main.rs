struct Point {
    x: i32,
    y: i32,
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

enum Color {
    Rgb(u8, u8, u8),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
    ChangeColor2(Color),
}

enum OneMessage {
    Hello { id: i32 },
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
        _ => (),
    }

    let msg = Message::ChangeColor2(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor2(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        Message::ChangeColor2(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: 10 });

    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        },
        _ => {
            setting_value = new_setting_value;
        },
    }

    println!("Setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }

    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    let origin = Point3D { x: 0, y: 0, z: 0 };

    match origin {
        Point3D { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    let msg = OneMessage::Hello { id: 5 };

    match msg {
        OneMessage::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        OneMessage::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        OneMessage::Hello { id } => println!("Found some other id: {}", id),
    }
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y paramater: {}", y);
}
