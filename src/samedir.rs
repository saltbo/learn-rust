pub fn longest1(x: &str) -> &str {
    x
}

pub fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    print!("{}", y);
    x
}