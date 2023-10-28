use std::fs;

fn main() {
    let file = fs::File::open("./examples/file_metadata.rs").unwrap();
    let metadata = file.metadata().unwrap();
    dbg!(&metadata.file_type());
}
