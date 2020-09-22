struct Point<T, V> {
    x: T, 
    y: V
}

struct Line<T, V> {
    start: Point<T, V>,
    end: Point<T, V>
}

pub fn generics() {
    let point1: Point<i32, f64> = Point { x: 0, y: 0.4 };
    let point2: Point<i32, f64> = Point { x: 0, y: 0.4 };

    let line: Line<i32, f64> = Line { start: point1, end: point2 };
}