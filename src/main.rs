extern crate diff_sync;


fn main() {
    // let a = "Hello hello you know I say a lot of things and this is one of them";
    // let b = "Hello hello well I say a lot of things but this is one of them";
    let c = "Hello helo abracadabra hocus pocus";
    let d = "Helko helko abracadabra hopus popus";
    // let update1 = diff_sync::word::compare(a, b);
    let update2 = diff_sync::word::compare(c, d);
    println!("To change: {:?}", update2);
    // let new = diff_sync::character::update(a.to_string(), &update1, 0);
    // println!("Original: {}, Change: {}, Synced: {}", a, b, new);
    // println!("Original: {}, Change: {}, Synced: {:?}", c, d, update2);
}