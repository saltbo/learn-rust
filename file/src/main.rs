fn main() {
    println!("Hello, world!");

    let f = std::fs::read_to_string("testdata/file.txt");
    if f.is_err() {
        println!("{}", f.err().unwrap());
        return;
    }
    // if let Err(err) = f {
    //     println!("{:?}", err);
    //     return;
    // }

    // println!("{}", f.unwrap());


    let r = std::fs::write("test.txt", "aaa");
    println!("{:?}", r);
    if r.is_err() {
        return;
    }

    // println!("{:?}", r.ok());

    match save() {
        Err(_) => {}
        Ok(_) => {}
    }

    // let file = Ok(f);
    // println!("{:?}", f);
}

fn save() -> std::io::Result<()> {
    std::fs::write("test2.txt", "aaa")?;
    Ok(())
}
