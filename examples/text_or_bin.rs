use std::{fs, io::Read};

fn main() {
    let text = "Hello world!".as_bytes();
    dbg!(text);

    let utf8 = String::from_utf8(text.into());
    dbg!(&utf8);

    let mut code = fs::File::open("./await_map.rs").unwrap();
    let mut buffer = [0u8; 8];
    code.read_exact(&mut buffer).unwrap();
    dbg!(&buffer);

    let code = String::from_utf8(buffer.into()).unwrap();
    dbg!(code);
}
