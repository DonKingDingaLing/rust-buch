use hex::encode;

fn main() {
    let hex_world = encode("Hello, world!");
    println!("{}", hex_world);
}
