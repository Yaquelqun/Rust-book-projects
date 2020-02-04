use std::io;

pub use crate::back::command_parser::parse;
use crate::structs::Request;

pub fn listen(mut request: Request) {
  while 1 == 1 {
    println!("#####################################################################");
    println!("Please enter your command");
    
    let mut command = String::new();
    io::stdin().read_line(&mut command)
    .expect("Failed to read line");
    request.raw = command;
    parse(&request);

  }
}