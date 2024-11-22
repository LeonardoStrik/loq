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
// use repl::Repl;
#[allow(dead_code)]
pub mod tui_repl;
use tui_repl::draw_tui;
mod test;

#[allow(dead_code)]
#[allow(unused_mut)]
fn main() -> io::Result<()> {
    //     let mut repl = Repl::new();
    //     repl.run()?;
    //     Ok(())
    draw_tui()
}
