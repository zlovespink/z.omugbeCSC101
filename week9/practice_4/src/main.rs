use std::fs::OpenOptions;
use std::io::Write;

fn main() {

    let mut file = OpenOptions::new().append(true).open("data.txt").expect(
        "cannot open file");
        file.write_all("\nhello Class".as_bytes()).expect("write failed");
        file.write_all("\nThis is the appendage to the document."
    .as_bytes()).expect("write failed");
    println!("file append succes");

}