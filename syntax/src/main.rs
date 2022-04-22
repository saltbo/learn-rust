#[derive(Debug)]
#[allow(dead_code)]

// Basic C-like enum
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

// Enum with fields
#[derive(Debug)]
enum OptionalI32 {
    AnI32(i32),
    Nothing,
}

#[allow(dead_code)]
fn main() {
    learn_var();
    learn_const();
    learn_enum();
    learn_array();
    learn_ifelse();
    learn_struct();
}

// 变量
fn learn_var() {
    let aaa: i32;
    aaa = 0;
    let abc = 1;
    let xx: i32 = 133;
    let str = "abc";
    println!("{},{},{},{}", aaa, abc, xx, str);
}

// 常量
fn learn_const() {
    const AAA: i32 = 1;
    println!("{}", AAA);
}

// 枚举
fn learn_enum() {
    let up = Direction::Up;
    let a = OptionalI32::AnI32(10);
    let b = OptionalI32::Nothing;
    println!("{:?},{:?},{:?},{:?}", up, OptionalI32::Nothing, a, b);
}

fn learn_array() {
    let mut array: [i32; 4] = [1, 2, 3, 4];
    array[0] = 44;
    println!("{:?}", array);
    for item in array.iter() {
        println!("{}", item)
    }

    let mut vec: Vec<&str> = vec![];
    println!("{:?}", vec);
    vec.push("123");
    println!("{:?}", vec);
    vec[0] = "222";
    println!("{:?}", vec);
}

fn learn_ifelse() {
    let value = if true { "good" } else { "bad" };
    println!("{:?}", value);

    if true {
        println!("true");
        33 // 分支的最后一条代码计算结果赋值给x，不能分号结尾
    } else {
        println!("false");
        44 // 分支的最后一条代码计算结果赋值给x，不能分号结尾
    };
    let x = 1;
    println!("{}", x);
}

fn learn_struct() {
    // Struct
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    // A struct with unnamed fields, called a ‘tuple struct’
    #[derive(Debug)]
    struct Point2(i32, i32);

    let mut origin: Point = Point { x: 0, y: 0 };
    origin.x = 1;
    origin.y = 12;

    let mut origin2 = Point2(0, 0);
    origin2.0 = 13;

    println!("{:?},{:?}", origin, origin2);
}
