use lexer::Tokens;

mod reader;
mod lexer; 

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let input_code:String = reader::read(); 
    let tokens:Vec<Tokens> = lexer::lex(input_code); 
    for t in tokens{
        println!("{:?}", t)
    }
}
