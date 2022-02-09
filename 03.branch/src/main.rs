fn main() {
    println!("Hello, world!");

    let number = 3;
    if number < 5 {
        print!("Value is less than 5");
    }else{
        println!("Value is greater than 5");
    }

    let number = if  number >5 {10} else { 0 };
}
