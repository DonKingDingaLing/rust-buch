fn main() {
    let wiederholung = [3.141; 10];
    let ganzzahl = 2;
    let mut feld = [ganzzahl,3, 5];
    println!("{:?}", feld);
    feld[1] = 4;
    let [mut a, _, b ] = feld;
    println!("{:#?}, {}, {}", feld, a, b);
}
