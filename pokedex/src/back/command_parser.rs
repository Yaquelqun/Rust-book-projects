use crate::structs::Request;
pub fn parse(mut request: &mut Request) { 
  request.raw = request.raw.trim().to_string();
  let mut vector = Vec::new();
  for word in request.raw.split_whitespace() { vector.push(String::from(word))};
  iter_request(vector, &mut request);
}

  fn iter_request(word_list: Vec<String>, request: &mut Request) {
     request.parsed = match &word_list[0].to_uppercase().as_ref() {
      &"ADD" => [String::from("ADD"), String::from(&word_list[1]), String::from(&word_list[word_list.len() - 1])],
      &"LIST" => [String::from("LIST"), String::from(""), String::from(&word_list[word_list.len() - 1])],
      _ => [String::from("QUIT"), String::from(""), String::from("")]
    };
  }