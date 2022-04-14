// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

mod package;
mod samedir;

use crate::package::user::*;
use crate::samedir::*;

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

    let aaa: i32;
    let abc = 1;
    let xx: i32 = 133;

    // String
    let str = "abc";

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

    // Basic C-like enum
    enum Direction {
        Left,
        Right,
        Up,
        Down,
    }

    let up = Direction::Up;

    // Enum with fields
    enum OptionalI32 {
        AnI32(i32),
        AAA,
        Nothing,
    }

    OptionalI32::AAA;
    // bff
    // 接口代理，静态文件挂载，API聚合，鉴权，权限控制，限流，熔断降级
}
