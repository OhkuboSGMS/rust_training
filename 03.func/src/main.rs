fn main() {
    println!("Hello, world!");
    other_func(23);

    let y ={
        let x =3;
        x+1
    };
    println!("Eval :{}", y);
    println!("Five :{}", five());
}

fn other_func(x:i32){
    println!("Hello Guys! -> {}",x);
}
fn five() -> i32 {
    5
}


// 自分のコメント
// これしか種類はない？