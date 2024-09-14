struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的字段
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let top_left: Point = Point { x: 5.2, y: 9.0 };
    let bottom_right = Point { x: 10.3, y: 3.0 };

    let rectangle = Rectangle {
        // 结构体的实例化也是一个表达式
        top_left,
        bottom_right,
    };

    println!("{}", rect_area(&rectangle));
}

fn rect_area(rect: &Rectangle) -> f32 {
    let Rectangle {
        top_left: Point {
            x: x1,
            y: y1,
        },
        bottom_right: Point {
            x: x2,
            y: y2,
        },
    } = rect;
    
    (x2-x1) * (y1-y2)
}
