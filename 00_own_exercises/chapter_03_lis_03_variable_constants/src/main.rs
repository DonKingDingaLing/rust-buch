
const KONSTANTE : i32 = 3;

fn main() {
    println!("{}", KONSTANTE);
    {
        const KONSTANTE:i32 = 4;
        println!("{}", KONSTANTE);
    }
    println!("{}", KONSTANTE);
}
