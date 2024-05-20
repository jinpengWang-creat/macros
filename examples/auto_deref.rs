use macros::AutoDeref;

#[allow(unused)]
#[derive(AutoDeref)]
#[deref(mutable = true, field = "inner")]

pub struct RespBulkString {
    inner: String,
    nothing: (),
}

fn main() {}
