// Variations of a same type
enum IpAddrKind {
    V4,
    V6,
}

// Can replace a struct to fill in multiple fields at once
enum IpAddrEx {
	V4(u8, u8, u8, u8),
	V6(String),
}

// 
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// impl Message {
//     fn call(&self) {
//         // method body would be defined here
//     }
// }

// No NULL values in Rust
// Instead, Option<T> enum encodes a value as present or absent
// Included in the prelude, doesn't need to be brought in scope
enum Option<T> {
    Some(T),
    None,
}

fn main() {
	// Create instances of each variations of the enum IpAddrKind
	// They still have the same type
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Kill two birds with one stone 
	let home = IpAddrEx::V4(127, 0, 0, 1);
	let loopback = IpAddrEx::V6(String::from("::1"));

	// Use of methods with enum
	let m = Message::Write(String::from("hello"));
	//m.call();

	// Option<T> enum examples
	let some_number = Some(5);
	let some_string = Some("a string");

	//let absent_number: Option<i32> = None;
}
