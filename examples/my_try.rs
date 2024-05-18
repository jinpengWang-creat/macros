use anyhow::Result;
use macros::{my_try, my_vec};

fn main() -> Result<()> {
    let v: Vec<i32> = my_vec![
        my_try!("1".parse()),
        my_try!("2".parse()),
        my_try!("3".parse()),
        my_try!("4".parse()),
    ];

    println!("{:?}", v);
    Ok(())
}
