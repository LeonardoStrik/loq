#[allow(dead_code)]
#[allow(unused_mut)]
mod repl;
use std::{env, io};

use repl::Repl;

#[allow(dead_code)]
#[allow(unused_mut)]
fn main() -> io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");

    let mut repl = Repl::new();
    repl.run()?;
    Ok(())
}
