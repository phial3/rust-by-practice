
/// 多边形
trait Polygon {
    fn area(&self) -> u32;

    fn new(height: u32, width: u32) -> Self;
}

/// 矩形
struct Rectangle {
    height: u32,
    width: u32,
}

/// 实现多边形
impl Polygon for Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn new(height: u32, width: u32) -> Self {
        Rectangle { height, width }
    }
}

/////////////////////////////////
/////////////////////////////////

/// 水果
enum Fruits {
    Apple,
    Banana,
    Strawberry,
    Acai,
}

/// 坐标
enum Coordinates {
    TwoCoordinates(i32, i32),
    ThreeCoordinates(i32, i32, i32),
}

/// 人
struct Person {
    name: String,
    age: u8,
    height: f32,
}

/// 列举所有水果
fn enumeration(fruit: Fruits) {
    match fruit {
        Fruits::Acai => println!("it's a acai"),
        Fruits::Apple => println!("it's an apple"),
        Fruits::Banana => println!("it's a banana"),
        Fruits::Strawberry => println!("it's a strawberry"),
    }
}

/// 列举所有坐标
fn match_coordinates(coordinates: Coordinates) {
    match coordinates {
        Coordinates::TwoCoordinates(x, y) => println!("Coord 2d ({}, {})", x, y),
        Coordinates::ThreeCoordinates(x, y, z) => println!("Coord 3d ({}, {}, {})", x, y, z),
    }
}

///
fn structure() {
    let eduardo = Person {
        name: "Edduardo".to_string(),
        age: 25,
        height: 1.7,
    };

    println!("{} is {} years old and {} meters tall", eduardo.name, eduardo.age, eduardo.height);
}

fn rectangle_area(rect: Rectangle) {
    println!("Rectangle area: {}", rect.area());
}

fn main() {
    enumeration(Fruits::Apple);
    enumeration(Fruits::Banana);
    enumeration(Fruits::Strawberry);
    enumeration(Fruits::Acai);

    let coord2d = Coordinates::TwoCoordinates(5, 2);
    let coord3d = Coordinates::ThreeCoordinates(5, 2, 10);
    match_coordinates(coord2d);
    match_coordinates(coord3d);

    structure();

    rectangle_area(Rectangle::new(5, 10))
}
