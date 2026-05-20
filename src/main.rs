mod lexer;
mod parser;
const SOURCE: &str = "N: u8 = 0.9
fn foo(bar: u32) u32 {
  print(bar)
  bar = bar >> 1
  return bar
}
fn main(){
  tree: u32
  flag: bool
  foo: u8[10][10]
  input(tree)
  flag = input > 100
  if !flag {
    print(\"END\")
  }else{
    print(N)
  }
}
";
fn main() {
    println!("{}",SOURCE);
    let mut lexer = lexer::Lexer::new(SOURCE);
    lexer.scan_src();
}
