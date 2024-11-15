fn main() {
    #[derive(Debug)]
    enum Wert {
        Ganzzahl(i32),
        Fliesskomma(f64),
        Tupel((i32, f64))
    }

    let wert = Wert::Ganzzahl(3);
    println!("Der Wert ist {:?}", wert);
}
