#![deny(clippy::all)]

use std::f32::consts::PI;

#[derive(Debug)]

enum Pet {
    Cat { name: String },
    Dog { name: String },
}
enum Shapes {
    Circle {
        radius: f32,
        center: (f32, f32),
    },
    Rectangle {
        width: f32,
        heigth: f32,
        center: (f32, f32),
    },
}

impl Shapes {
    fn area(&self) -> f32 {
        match self {
            Shapes::Circle { radius, .. } => PI * radius.powi(2),
            Shapes::Rectangle { width, heigth, .. } => heigth * width,
        }
    }
    fn is_a_point_inside(&self, point: (f32, f32)) -> bool {
        match self {
            Shapes::Circle { radius, center } => {
                let distance_from_origin =
                    ((center.0 - point.0).powi(2) + (center.1 - point.1).powi(2)).sqrt();
                distance_from_origin < *radius
            }
            Shapes::Rectangle {
                width,
                heigth,
                center,
            } => {
                let (x_0, x_f) = (center.0 - width / 2.0, center.0 + width / 2.0);
                let (y_0, y_f) = (center.1 - heigth / 2.0, center.1 + heigth / 2.0);
                println!("x0:{}\nxf:{}\ny0:{}\nyf:{}", x_0, x_f, y_0, y_f);
                !(x_0 > point.0 || x_f < point.0 || y_0 > point.1 || y_f < point.1)
            }
        }
    }
}
fn main() {
    let rectangle = Shapes::Rectangle {
        width: 2.0,
        heigth: 2.0,
        center: (0.0, 0.0),
    };

    println!("Rectangle area {}", rectangle.area());

    let circle = Shapes::Circle {
        radius: 4.0,
        center: (0.0, 0.0),
    };
    println!("Circle area {}", circle.area());

    let point: (f32, f32) = (0.5, 0.5);
    let is_inside = rectangle.is_a_point_inside(point);
    if is_inside {
        println!("ta dentro")
    } else {
        println!("ta fora")
    }
    let point: (f32, f32) = (5.0, -5.6);

    let is_inside = circle.is_a_point_inside(point);
    if is_inside {
        println!("ta dentro do circulo")
    } else {
        println!("ta fora do circulo")
    }
}
/*
#[cfg(test)]
mod tests {} */
