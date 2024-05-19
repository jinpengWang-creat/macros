use anyhow::Result;

fn main() -> Result<()> {
    let v: Vec<i32> = vec![
        my_try!("1".parse()),
        my_try!("2".parse()),
        my_try!("3".parse()),
        my_try!("4".parse()),
    ];

    println!("{:?}", v);
    Ok(())
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
