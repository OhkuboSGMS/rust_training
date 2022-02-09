fn main() {
    println!("Hello, world!");
    let mut s = String::from("hello");
    println!("{}", s);
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1 = {} , s2 = {} ", s1, s2);

    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;
    makes_copy(x);
    println!("After Copy {}", x);

    let s3 = gives_ownership();
    let s4 = String::from("おはようー");
    let s5 = takes_and_gives_back(s4);
    println!("s5 {}", s5);

    let (s6, len) = calc_len(s5);

    println!("length of s6 {}", len);

    println!("length of s6-2 {}",calc_len2(&s6));
    println!("s6 {}",s6);
    let mut s6 = s6;
    change(&mut s6);
    println!("change s6 {}",s6);

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn gives_ownership() -> String {
    let some = String::from("こんにちわ");
    some
}

fn takes_and_gives_back(a_str: String) -> String {
    a_str
}

fn makes_copy(some_integer: i32) {
    println!("{} ", some_integer)
}

fn calc_len(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calc_len2(s: &String) -> usize {
    s.len()
}

fn change(some: &mut String){
    some.push_str("+Alpha");
}