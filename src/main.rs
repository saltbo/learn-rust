// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

mod samedir;
mod package;

use crate::samedir::*;
use crate::package::user::*;

fn main() {
    // let x = "abc";
    // let y = "abc";
    // longest1(x);
    // longest2(x, y);

    let mut user1 = User::new("someusername123", "someone@example.com", 1, true);
    println!("{:?}", user1);
    user1.nickname = "test";
    println!("{:?}", user1);

    user1.updateUsername("aaa");
    println!("{:?}", user1);
}
