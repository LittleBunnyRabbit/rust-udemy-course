enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), // touple
    CmykColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8
    } // struct
}

pub fn enumerations() {
    let c: Color = Color::CmykColor{ cyan: 1, magenta: 2, yellow: 3, black: 255 };

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0) 
        | Color::CmykColor{ black: 255, .. } => println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb ({}, {}, {})", r, g, b),
        _ => ()
    }
}