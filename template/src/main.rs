use std::io;

fn main() {
    let mut dst: String = String::new();
    io::stdin().read_line(&mut dst).expect("failed to read line");
    println!("echo: {}", dst);
}
