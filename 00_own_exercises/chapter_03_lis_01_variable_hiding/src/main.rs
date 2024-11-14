fn main() {
    let nur_lesen : i32;
    nur_lesen = 3;
    // nu_lesen = 4; // kann nicht compiled werden
    let mut veraenderbar = 5;
    veraenderbar += 5;
    println!("{}", veraenderbar);

    {
        let veraenderbar = veraenderbar * 2;
        println!("{}", veraenderbar);
    }
    veraenderbar += 1;
    println!("{}", veraenderbar);
}
