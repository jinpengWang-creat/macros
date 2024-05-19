use anyhow::Result;

fn main() -> Result<()> {
    let v: Vec<i32> = my_vec![
        "1".parse()?,
        "2".parse()?,
        "3".parse()?,
        "4".parse()?,
        "5".parse()?,
        "6".parse()?,
    ];

    println!("{:?}", v);
    Ok(())
}

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
