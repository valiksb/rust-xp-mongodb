mod error;

pub use self::error::{Error, Result};

fn main() -> Result<()> {
    println!("Hello, this world!");
    Ok(())
}
