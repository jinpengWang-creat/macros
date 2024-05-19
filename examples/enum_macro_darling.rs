use macros::EnumFromDarling;

#[allow(dead_code)]
#[derive(Debug, EnumFromDarling)]
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
    let up: Direction<_> = DirectionUp::new(String::from("hello world")).into();
    println!("{:?}", up);
}
