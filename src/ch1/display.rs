use std::fmt;

#[derive(Debug)]
struct MinMax(i32, i32);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: i32,
    y: i32,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x:{},y:{}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f32,
    imag: f32,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

pub fn display() {
    let minmax = MinMax(1, 2);
    println!("Display MinMax:{}", minmax);
    println!("Debug MinMax: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 1, y: 2 };
    println!("Display Point2D:{}", point);
    println!("Debug Point2D: {:?}", point);

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };

    println!("Display Complex:{}", complex);
    println!("Debug Complex: {:?}", complex);
}
