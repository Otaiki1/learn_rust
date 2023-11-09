fn main() {
    println!("Hello, world!");
    for i in 1..5{
        println!("{}", i);
    }

    let x  = 1..4;
    for ii in x{
        println!("2nd -- {}", ii);
    }

    let fruits = vec!["Apple", "Oranges", "mangoes"];
    // for ii in fruits{
    //     println!("A value is {}", ii)
    // }
    for (index, item) in fruits.iter().enumerate(){
        println!("The {} fruit is {}", index, item );
    }
    // MiGRATES TO WHILE 
    let mut number: i32= 0;
    while number <=20{
        println!("The while loop  number is {}", number );
        number +=1;

    }
}
