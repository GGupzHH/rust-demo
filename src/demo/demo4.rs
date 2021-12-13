use std::fmt::{self, Formatter, Display};

pub struct City {
    // 8个字节
    pub name: &'static str,
    // 4
    pub lat: f32,
    pub lon: f32
}

#[derive(Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

struct Point2D {
    pub x: u8,
    y: u8
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lat >= 0.0 { 'E' } else { 'W' };

        write!(f, "{}: {:.3}°{} {:.3}°{}",
            self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Red-->{}, Green-->{}, Blue-->{},",
            self.red, self.green, self.blue)
    }
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x -> {}, y -> {}", self.x, self.y)
    }
}

pub fn point2_d_fn () {
    let point = Point2D { x: 1, y: 2 };
    
    println!("{}", point);
}
