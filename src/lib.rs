use std::fmt;
use std::mem;

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
    Blue,
    Rgb(u8, u8, u8), // tuple
    Cmyk { cyan: u8, magenta: u8, yellow: u8, black: u8 } // struct
}

fn print_color(color: &Color) {
    match color {
        Color::Red => println!("red"),
        Color::Green => println!("red"),
        Color::Blue => println!("blue"),
        Color::Rgb(0, 0, 0)
        | Color::Cmyk { cyan: _, magenta: _, yellow: _, black: 255 } => println!("black"),
        Color::Rgb(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
        Color::Cmyk { cyan: c, magenta: m, yellow: y, black: b } => println!("cmyk({}, {}, {}, {})", c, m, y, b),
    }
}

pub fn stack_and_heap() {
    let point = Box::new(Point { x: 0.0, y: 0.0 });
    println!("point reference takes up {} bytes", mem::size_of_val(&point));
    println!("point takes up {} bytes", mem::size_of_val(&*point));

    println!("point reference has the value {}", point);
    println!("point has the value {}", *point);

    let red = Color::Red;
    println!("enum value red has {} bytes", mem::size_of_val(&red));

    let rgb = Color::Rgb(255, 0, 168);
    println!("enum value rgb has {} bytes", mem::size_of_val(&rgb));
    print_color(&rgb);

    let cmyk = Color::Cmyk{ cyan: 254, magenta: 0, yellow: 255, black: 15 };
    println!("enum value cmyk has {} bytes", mem::size_of_val(&cmyk));
    print_color(&cmyk);

    let rgb_black = Color::Rgb(0, 0, 0);
    println!("enum value rgb_black has {} bytes", mem::size_of_val(&rgb_black));
    print_color(&rgb_black);

    let cmyk_black = Color::Cmyk{ cyan: 254, magenta: 0, yellow: 255, black: 255 };
    println!("enum value cmyk_black has {} bytes", mem::size_of_val(&cmyk_black));
    print_color(&cmyk_black);
}
