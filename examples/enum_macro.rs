use macros::EnumFrom;

#[allow(dead_code)]
#[derive(Debug, EnumFrom)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(i32),
}
#[allow(dead_code)]
#[derive(Debug)]
struct DirectionUp {
    speed: u32,
}

fn main() {
    let up: Direction = 11_i32.into();
    println!("{:?}", up);
}
