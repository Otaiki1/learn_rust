fn main() {
    let s_tuple=("where", "when", "how");
    let i_tuple=(1,2,3,4,5);

    let mixedTuple = ("OT", 1, 32.2, 'x');
    println!("The string is {}, integer is {}", mixedTuple.0, mixedTuple.1);

    let mixedTuple = ("OT", 1, 32.2, 'x',("where", "when", "how"));
    println!("The string is {}", (mixedTuple.4).0);
}
