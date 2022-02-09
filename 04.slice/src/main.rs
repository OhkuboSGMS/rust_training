fn main() {
    let mut msg = String::from("Helloworld");
    let word = first_word(&msg);
    msg.clear();

    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // s.clear();

    println!("the first word is: {}", word);
    // let hello = &msg[0..5];
    // let world = &msg[6..11];
    //
    // println!("hello {}",hello);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}