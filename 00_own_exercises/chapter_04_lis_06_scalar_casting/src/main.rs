fn main() {
    let ganzzahl = 3_i32;
    // let fp:f64 = ganzzahl; // Fehler
    let fp = ganzzahl as f64;
    let c = true as u32;
    println!("{}", c);
}
