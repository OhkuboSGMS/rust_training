fn main() {
    println!("Hello, world!");
    let dangle1 = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
