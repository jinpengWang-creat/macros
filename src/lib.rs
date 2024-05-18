/// my_vec
#[macro_export]
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($($x:expr),+$(,)?) => {
        <[_]>::into_vec(Box::new([$($x),*]))
    };
    ($elem:expr; $n:expr) => {
        std::vec::from_elem($elem, $n)
    }
}

/// use ? operator, how to simulte it ?
#[macro_export]
macro_rules! my_try {
    ($elem:expr) => {
        match $elem {
            Ok(val) => val,
            Err(e) => return Err(e.into()),
        }
    };
}
