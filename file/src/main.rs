use std::fmt::Error;

fn main() {
    println!("Hello, world!");

    // let r = std::fs::write("test.txt", "aaa");
    // println!("{:?}", r);
    // if r.is_err() {
    //     return;
    // }

    // println!("{:?}", r.ok());

    match save() {
        Err(_) => {}
        Ok(_) => {}
    }

    // let file = Ok(f);
    // println!("{:?}", f);


    // match f {
    //     Ok(file) => {
    //         println!("{:?}", file);
    //         println!("File opened successfully.");
    //     }
    //     Err(err) => {
    //         println!("Failed to open the file.");
    //     }
    // }
}

fn save() -> std::io::Result<()> {
    std::fs::write("test2.txt", "aaa")?;
    Ok(())
}
