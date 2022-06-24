use std::sync::mpsc::channel;
use std::sync::Mutex;
use std::thread;
use std::thread::spawn;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let h = thread::spawn(|| {
        println!("1");
        thread::sleep(Duration::from_secs(1));
        println!("2");
    });

    let str = "hello";
    thread::spawn(move || {
        println!("{},3", str);
        thread::sleep(Duration::from_secs(5));
        println!("{},4", str);
    });

    println!("0{}", 0);
    thread::sleep(Duration::from_secs(2));
    println!("{}", str);
    thread::sleep(Duration::from_secs(2));
    println!("0{}", 1);

    h.join();

    let (tx, rx) = channel();
    spawn(move || {
        tx.send(123);
        thread::sleep(Duration::from_secs(1));
        tx.send(2333);
        tx.send(123);
        tx.send(123);
        tx.send(123);
        tx.send(123);
        tx.send(123)
    });

    for received in rx.iter() {
        println!("Got: {}", received);
    }

    #[derive(Debug)]
    struct Manager<'a> {
        n: i32,
    }

    impl Manager<'a> {}

    let mut m = &mut Manager { n: 10 };
    (*m).n = 1;
    thread::spawn(move || {
        (*m).n = 12;
    });

    println!("{:?}", m);

    // mutex
    let m = Mutex::new(234);
    let mut num = m.lock().unwrap();
    *num = 111;
    println!("{}", num);
}

