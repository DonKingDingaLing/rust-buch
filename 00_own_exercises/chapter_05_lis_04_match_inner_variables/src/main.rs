fn main() {
    let a = 3;

    let ausgabe = match a {
        a_innen if a_innen < 5 => a_innen + a_innen,
        a_innen => a_innen * a_innen,
    };

    println!("{}", ausgabe);
}
