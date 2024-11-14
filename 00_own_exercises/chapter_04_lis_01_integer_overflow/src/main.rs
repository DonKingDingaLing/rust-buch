fn main() {
    let mut wert : i16 = 32767;
    // wert += 1; // Fehler beim compilen
    let ueberlauf = wert.wrapping_add(1);
    let gesaettigt = wert.saturating_add(1);

    println!("{}", ueberlauf);
    println!("{}", gesaettigt);
}
