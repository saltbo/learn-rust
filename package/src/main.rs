mod package;
mod samedir;

use crate::package::user::*;

fn main() {
    println!("Hello, world!");
    samedir::longest1("abc");
    samedir::longest2("aaa", "bbb");
}
