use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let x = 5;
    let b = Box::new(x);

    let y = 7;
    let m = MyBox::new(y);

    assert_eq!(5, *b);
    assert_eq!(7, *m);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
