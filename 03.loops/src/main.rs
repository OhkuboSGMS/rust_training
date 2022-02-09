fn main() {
    for1();
}

fn for1() {
    let a = [10, 20, 30, 40, 60];

    for element in a.iter() {
        println!("value {}", element);
    }
}

fn loop1() {
    loop {
        println!("Hello, world!");
    }
}

fn while1() {
    let mut num = 0;

    while num < 100 {
        println!("i {}", num);
        num += 1;
    }
}

