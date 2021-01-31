use std::fs;

fn main() {
    let files = fs::read_dir("./").unwrap();

    for file in files {
        println!("{}", file.unwrap().path().display());
    }
}
