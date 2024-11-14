fn main() {
    let mut temperature = 10;

    while temperature <= 21 {
        if temperature == 21 {
            break
        }
        println!("Heizung einschalten");
        temperature += 1;

        if temperature < 16 {
            continue;
        }
        println!("Es ist kalt")
    };

    println!("Heizung abschalten");
}
