use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    let (ty, ry) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("spawned"),
            String::from("thread!"),
        ];

        for val in vals {
            ty.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in ry {
        println!("Got: {}", received);
    }

    let (tz, rz) = mpsc::channel();

    let tz1 = tz.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("third"),
            String::from("spawned"),
            String::from("thread!"),
        ];

        for val in vals {
            tz1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("More"),
            String::from("messages"),
            String::from("for"),
            String::from("you!"),
        ];

        for val in vals {
            tz.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in rz {
        println!("Got: {}", received);
    }
}
