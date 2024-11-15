fn main() {
    let a = 3;

    let ausgabe = match a {
        a_innen @ 0..=4 => a_innen + a_innen,
        a_innen => a_innen * a_innen,
    };

    println!("{}", ausgabe);
}
