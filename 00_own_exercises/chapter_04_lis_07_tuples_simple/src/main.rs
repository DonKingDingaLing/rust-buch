fn main() {
    let buchstabe = 'r';
    let mut tupel = (buchstabe, 3, 3.1);
    println!("{:?}", tupel);
    tupel.1 = 4;
    let (a, _, mut b) = tupel;
    println!("{:#?}, {}", tupel, a);
}
