use macros::MyError;

#[derive(Debug, MyError)]
pub enum MyError {
    Io(#[my_error(from)] std::io::Error),

    Parse {
        #[my_error(from)]
        from: std::num::ParseIntError,
    },
    Custom {
        #[my_error(from)]
        error_info: String,
    },
}

fn main() {
    let error: MyError = String::from("This is a Custom Error").into();
    println!("{:?}", error);
}
