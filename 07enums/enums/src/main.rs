enum Gender  {
    Male,
    Female,
    Others
}

fn main() {
    let person:Gender   = Gender::Male;

    match person{
        Gender::Male => println!("Person is Male"),
        Gender::Female => println!("Person is Female"),
        Gender::Others => println!("Person is Others")
    }
}
