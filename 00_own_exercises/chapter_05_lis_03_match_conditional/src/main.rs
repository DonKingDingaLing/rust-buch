fn main() {
    let a = 3;
    let flag = false;
    
    let ausgabe = match a {
        3 if flag => "3",
        3 ..= 5 => {"Zwischen 3 und 5"},
        _ => "Anderer Wert",
    };
    
    println!("{}", ausgabe);
}
