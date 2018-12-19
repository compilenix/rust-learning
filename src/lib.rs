use std::mem;
use std::fmt;

struct Point {
    x: f64,
    y: f64,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "{{ x: {}, y: {} }}", self.x, self.y)
    }
}

enum Color {
    Red,
    Green,
    Blue
}

pub fn stack_and_heap() {
    let point = Box::new(Point { x: 0.0, y: 0.0 });
    println!("point reference takes up {} bytes", mem::size_of_val(&point));
    println!("point takes up {} bytes", mem::size_of_val(&*point));

    println!("point reference has the value {}", point);
    println!("point has the value {}", *point);

    let red = Color::Red;
    println!("enum value red has {} bytes", mem::size_of_val(&red));
}
