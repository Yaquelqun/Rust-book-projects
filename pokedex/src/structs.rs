use std::collections::HashMap;

pub struct Request {
  pub pokedex: HashMap<String, Vec<String>>,
  pub raw: String,
  pub parsed: [String; 3]
}