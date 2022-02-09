use std::fs::File;

fn main() {
    // let v = vec![1,2,3];
    // v[99];

    // let f:u32 = File::open("hello.txt");
    let f = File::open("hello.txt");

    let f = match f{
        Ok(file) => file,
        Err(error)=>{
            panic!("File Exception :{}",error)
        }
    };
}
