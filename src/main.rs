struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    // Can use . syntax
    fn area(&self) -> f32 {
        let height = self.top_left.y - self.bottom_right.y; 
        let width = self.bottom_right.x - self.top_left.x;
        height * width
    }

    // Use the :: syntax
    fn get_area(top_left: Point, bottom_right: Point) -> f32{
        let height: f32 = top_left.y - bottom_right.y;
        let width: f32 = bottom_right.x - top_left.x;
        height * width
    }
}

fn main() {
    // Rusts syntax for destructuring:
    // the match statment is how to 'destructure' tuples and enums
    // pointers are destructured with `&` `ref` and `mut ref`

    // destructureing in rust is not the same as it is in JS or PY.
    // the closest example is this Color struct:
    struct Color(u8, u8, u8);
    let Color(red, green, blue) = Color(55, 155, 255);
    println!("Red: {}, Green: {}, Blue: {}", red, green, blue);

    let rect = Rectangle {
        top_left: Point{ x: 20.0, y: 10.0 },
        bottom_right: Point{ x: 1.0, y: 2.0 },
    };

    // Can't do both because of `partial borrow`
    // println!("{}", Rectangle::get_area(rect.top_left, rect.bottom_right));
    println!("{}", rect.area());
}




