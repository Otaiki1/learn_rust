fn main() {
    let mut n  = 0;

    loop{
        n += 1;

        //would run infinitely if below statemnet is absent
        if n==7{
            continue;
        }
        if n>15{
            break;
        }
        println!("The value of n is {}", n);

    }
}
