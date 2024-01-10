use std::path::PathBuf;

fn main() {
    let absolute = PathBuf::from("123/skhd_xfy.pid");
    let relative = PathBuf::from("./tmp/test/123/skhd_xfy.pid");

    dbg!(&absolute
        .parent()
        .unwrap()
        .components()
        .collect::<Vec<_>>()
        .len());
    dbg!(&absolute.parent().iter().len());
    dbg!(&relative.parent().iter().len());
}
