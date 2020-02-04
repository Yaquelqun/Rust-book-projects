use crate::structs::Request;
pub fn parse(mut request: &mut Request) { 
  request.raw = request.raw.trim().to_string();
  let mut vector = Vec::new();
  for word in request.raw.split_whitespace() { vector.push(String::from(word))};
  iter_request(vector, &mut request);
  println!("you wrote: {:?}", request.parsed);
}

  fn iter_request(word_list: Vec<String>, request: &mut Request) {
     request.parsed = match word_list[0].to_uppercase() {
      "ADD" => ["ADD", word_list[1], word_list[word_list.len() - 1]],
      "LIST" => ["LIST", "", word_list[word_list.len() - 1]],
      _ => ["QUIT", "", ""]
    }
  }