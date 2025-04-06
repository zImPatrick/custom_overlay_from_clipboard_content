use std::fs;

fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("data.txt")
        .expect("Something went wrong data.txt");

    println!("With text:\n{contents}");
}
