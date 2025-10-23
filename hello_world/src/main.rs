fn main() {
    let name = std::env::args().nth(1).unwrap_or_else(|| String::from("world"));
    println!("Hello, {}!", name);
}
