#[derive(Debug)] // needed for println!({:?} OR {:#?})
struct Rectangle {
    width: u32,
    height: u32,
}

// You can have multiple "impl" blocks if you want
impl Rectangle {
	// Methods
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    // Method syntax
    println!("rect1 is {:?}", rect1);
    println!("The area of the rectangle is {} square pixels.", 
    	rect1.area());

    // No -> operator like in C or C++, instead
    // Rust does automatic referencing:
    //p1.distance(&p2); == (&p1).distance(&p2);


    //Multiple parameters methods
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    //Associated functions
    let sq = Rectangle::square(3); // same syntax as String::from()
    println!("sq is {:?}", sq);
}
