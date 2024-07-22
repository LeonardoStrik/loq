use std::io;

#[allow(dead_code)]
mod diag;
#[allow(dead_code)]
#[allow(unused_mut)]
mod expr;
#[allow(dead_code)]
#[allow(unused_mut)]
pub mod lexer;
#[allow(dead_code)]
#[allow(unused_mut)]
mod repl;
use repl::Repl;
mod test;

#[allow(dead_code)]
#[allow(unused_mut)]
fn main() -> io::Result<()> {
    let mut repl = Repl::new();
    repl.run()?;
    Ok(())
}
