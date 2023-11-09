fn add_two_nums(x:i64 , y:i64){
    let z  = x+y;
    println!("The resultof summing {} and {} is {}",x,y,z);
}
const string:&str = "bananarama";
fn main() {
    println!("Hello, world!");
    add_two_nums(23, 32);

   

    fn upper(){
        println!("Upper case is {}", string.to_ascii_uppercase());
    }

    upper()
}
