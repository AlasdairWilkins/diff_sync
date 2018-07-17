extern crate diff_sync;


fn main() {
    let a = "Hello helo abracadabra hocus pocus";
    let b = "Helko helko abracadabra hopus popus aliopus";
    let c = "Hello helo abracadabra hocus pocus";
    let d = "Helko helko abracadabra hopus popus aliopus";
    let update1 = diff_sync::character::compare(a, b);
    let update2 = diff_sync::word::compare(c, d);
    println!("To change: {:?}", update1);
    let new1 = diff_sync::character::update(a.to_string(), &update1, 0);
    let new2 = diff_sync::word::update(c.to_string(), &update2, 0);
    println!("Original: '{}', Change: '{}', Synced: '{}'", a, b, new1);
    println!("Original: '{}', Change: '{}', Synced: '{}'", c, d, new2);
}