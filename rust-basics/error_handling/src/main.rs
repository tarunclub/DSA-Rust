use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let f = File::open("hello.txt").unwrap();

    // let file = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file"),
    //         },
    //         other_error => {
    //             panic!("problem somewhere else")
    //         }
    //     },
    // };
}
