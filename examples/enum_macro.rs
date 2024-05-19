use macros::EnumFrom;

#[allow(dead_code)]
#[derive(Debug, EnumFrom)]
enum Direction<T> {
    Up(DirectionUp<T>),
    Down,
    Left(i32),
}
#[allow(dead_code)]
#[derive(Debug)]
struct DirectionUp<T> {
    speed: T,
}

impl<T> DirectionUp<T> {
    pub fn new(speed: T) -> Self {
        Self { speed }
    }
}

fn main() {
    let up: Direction<i32> = DirectionUp::new(12).into();
    println!("{:?}", up);
}
