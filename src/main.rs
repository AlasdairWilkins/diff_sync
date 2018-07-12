extern crate diff_sync;

fn main() {
    let a = "Hello";
    let b = "Oh Hello";
    let c = "Hello abracadabra hello";
    let d = "Helko abracadabra helko";
    diff_sync::compare(a, b);
    diff_sync::compare(c, d);
}