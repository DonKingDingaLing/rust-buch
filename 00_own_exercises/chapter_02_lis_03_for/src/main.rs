fn main() {
    for i in 0..10 {
        println!("{}", i);
    }

    let beginn = 1;
    let ende = 10;

    for i in beginn..=ende {
        println!("{}", i);
    }

    for i in 0.. {
        println!("{}", i);

        if i > 10 {
            break;
        }
    }
}
