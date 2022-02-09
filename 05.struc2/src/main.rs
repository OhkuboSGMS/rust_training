
struct  User{
    name:String,
    email:String,
    sign_in_count: u64,
    active: bool
}
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);
fn main() {
    let user1 = User{
        email: String::from("example@ja.com"),
        name: String::from("john"),
        sign_in_count:1,
        active:true,
    };
    println!("Hello, world! {}",user1.email);

    let black = Color(0,0,0);
}
