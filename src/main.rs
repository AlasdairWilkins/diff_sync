extern crate diff_sync;

fn main() {
    let a = "Hello";
    let b = "Hello";
    let c = "Hello hello abracadabra hocus pocus";
    let d = "Helko helko abracadabra hopus popus";
    let update1 = diff_sync::compare(a, b);
    let update2 = diff_sync::compare(c, d);
    println!("{:?}", update1);
    println!("{:?}", update2);
}