#[derive(Debug)]

struct Point {
    x: i32,
    y: i32,
}
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: &Rectangle) -> i32 {
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = rect;
    let width = x2 - x1;
    let height = y2 - y1;
    width * height
}

fn square(point: Point, length: i32) -> Rectangle {
    Rectangle {
        top_left: Point {
            x: point.x,
            y: point.y,
        },
        bottom_right: Point {
            x: point.x + length,
            y: point.y + length,
        },
    }
}

pub fn structs() {
    let point = Point { x: 1, y: 2 };
    let new_rect = square(point, 2);
    println!("{:?}", new_rect);
    println!("{}", rect_area(&new_rect));
}
