use std ::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize,f32, f64};

use std::io::stdin;
fn main() {
    println!("Hello, world!");

    let num:i8 = 7;//immutable by default
    println!("The number is {}", num);

    let name: &str = "Otaiki";
    println!("The naeme is {}", name);
    //single line comments

    //make immutable variable mutable
    let mut num = 100;
    println!("The number is now  {}", num);
    //booleans
    let is_right: bool  = true;

    //characters
    let one_char: char = 'a';

    //multiple variable declaratons
    let (country, state) = ("Nigeria", "Ekiti");

    //operators are same as always
}
