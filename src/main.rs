extern crate diff_sync;

fn main() {
    let a = "Hello";
    let b = "Helpo";
    let c = "Hello helo abracadabra hocus pocus";
    let d = "Helko helko abracadabra hopus popus";
    let update1 = diff_sync::compare(a, b);
    let update2 = diff_sync::compare(c, d);
    println!("Original: {}, Change: {}, Synced: {:?}", a, b, update1);
    println!("Original: {}, Change: {}, Synced: {:?}", c, d, update2);
}