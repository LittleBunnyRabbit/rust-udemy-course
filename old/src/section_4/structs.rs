struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

pub fn structs() {
    let p1 = Point { x: 3.0, y: 2.0 };
    println!("p1 = ({}, {})", p1.x, p1.y);

    let p2 = Point { x: 2.0, y: 4.0 };
    println!("p2 = ({}, {})", p2.x, p2.y);

    let line = Line { start: p1, end: p2 };
}