pub use command_impl::repl_command;

#[repl_command("quit"|"q",)]
fn run_command() {
    println!("Hello from my macro, Sailor!")
}
