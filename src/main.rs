use std::io;

#[allow(dead_code)]
#[allow(unused_mut)]
pub mod lexer;
#[allow(dead_code)]
#[allow(unused_mut)]
mod repl;

#[allow(dead_code)]
#[allow(unused_mut)]
fn main() -> io::Result<()> {
    // let some_string = String::from("(abc+1234+c)");
    // let mut buffer = " ".repeat(some_string.len());
    // let mut tokens: Vec<lexer::Token> = vec![];
    // let mut lexer = lexer::Lexer::from_string(some_string);
    // loop {
    //     if let Some(token) = lexer.peek_token() {
    //         println!("{}", token);
    //     }
    //     match lexer.next_token() {
    //         Some(token) => tokens.push(token),
    //         None => break,
    //     }
    // }
    // for token in tokens {
    //     print!("{}", token);
    //     buffer.insert(token.loc.col as usize, '^');
    // }
    // println!();
    // println!("{}", buffer);
    repl::repl()?;
    Ok(())
}
