#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let top_left: Point = Point { x: 5.2, y: 9.0 };
    let edge = 4.0;

    println!("{:#?}", square(&top_left, edge));
}

fn square(point: &Point, edge: f32) -> Rectangle {
    Rectangle {
        top_left: Point {
            x: point.x,
            y: point.y,
        },
        bottom_right: Point {
            x: point.x + edge,
            y: point.y - edge,
        },
    }
}
