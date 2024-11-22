use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn repl_command(args: TokenStream, input: TokenStream) -> TokenStream {
    eprintln!("{:#?}", args);
    eprintln!("-------------------------------------------");
    eprintln!("{:#?}", input);
    let _ = args;
    let _ = input;
    TokenStream::new()
}

// TODO: expr! macro?
