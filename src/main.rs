mod lexer;
mod parser;
const SOURCE: &str = "fn main(): u8 {
    x: u8 = 10
    y: f32 = 10 90.0.2
    print((x+y)/y<<2)
}";
fn main() {
    println!("{}",SOURCE);
    let mut lexer = lexer::Lexer::new(SOURCE);
    let mut parser = parser::Parser::new(&mut lexer);
    println!("{:?}", parser);
}
