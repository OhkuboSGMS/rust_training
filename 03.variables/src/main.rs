const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("The Value of x is :{}", x);
    x = 6;
    println!("The Value of x is :{}", x);

    let x = x + 1;
    println!("The Value of x is :{}", x);
    let x = x * 2;
    println!("The Value of x is :{}", x);

    let int: i32 = 32;
    println!("{}", int);

    let _10 = 98_222;
    let _h = 0xff;
    let _o = 0o77;
    let _b = 0b0000_0111;

    let t: bool = true;
    println!("{}", t);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("{},{},{}", x, y, z);
    println!("{}", tup.0);

    let ary = [1, 2, 3, 4];
    println!("{}", ary[0]);

}
