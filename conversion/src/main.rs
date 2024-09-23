///This is a copy of coversion from RustByExample
///https://doc.rust-lang.org/rust-by-example/conversion/string.html

//Macro
macro_rules! pi_f32 {
    () => {
        3.14159265358979323846264338327950288_f32
    };
}

struct Circle {
    radius: i32,
    area: f32,
}

impl Circle {
    fn area(radius: i32) -> Self {
        let radius_f32 = radius as f32;
        Circle {
            radius,
            area: radius_f32 * radius_f32 * pi_f32!(),
        }
    }
}

fn main() {
    let circle = Circle::area(5);
    println!("Circle with radius {} has area {}", circle.radius, circle.area);
}
