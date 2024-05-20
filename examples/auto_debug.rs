use macros::AutoDebug;

#[allow(unused)]
#[derive(AutoDebug)]

pub struct RespBulkString {
    inner: String,
    #[debug(skip = true)]
    nothing: (),
}

fn main() {
    let resp = RespBulkString {
        inner: String::from("hello world"),
        nothing: (),
    };
    println!("{:?}", resp);
}
