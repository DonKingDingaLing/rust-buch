struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let origin = Point {
        x: 0,
        y: 0,
    };
    println!("Point({}, {})", origin.x, origin.y);
}
