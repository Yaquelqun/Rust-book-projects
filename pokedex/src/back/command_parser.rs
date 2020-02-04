use crate::structs::Request;
pub fn parse( mut request: &Request) {
  print!("hello");
  println!("you entered {}", request.raw);
}