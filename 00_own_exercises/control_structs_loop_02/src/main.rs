fn main() {
    let mut temperature = 10;

    let text = loop {
      if temperature > 21 {
          break "Heizung abschalten"
      }
        println!("Heizung einschalten");
        temperature += 1;

        if temperature < 16 {
            continue;
        }
        println!("Es ist kalt")
    };

    println!("{}", text);
}
