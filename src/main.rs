extern crate diff_sync;

fn main() {
    let a = "<p>Why hello!</p>";
    let b = "<p>What's all this then?</p>";
    // let c = "Hello helo abracadabra hocus pocus";
    // let d = "Helko helko abracadabra hopus popus";
    let update1 = diff_sync::compare(a, b);
    // let update2 = diff_sync::compare(c, d);
    println!("To change: {:?}", update1);
    let new = diff_sync::update(a.to_string(), &update1, 0);
    println!("Original: {}, Change: {}, Synced: {}", a, b, new);
    // println!("Original: {}, Change: {}, Synced: {:?}", c, d, update2);
}