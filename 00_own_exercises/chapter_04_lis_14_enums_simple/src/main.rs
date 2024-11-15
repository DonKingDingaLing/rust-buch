fn main() {
    #[derive(Debug)]
    enum Ampel { Rot, Gelb, Gruen}
    
    let ampel = Ampel::Gruen;
    
    println!("Die Ampel steht auf {:?}", ampel);
}
