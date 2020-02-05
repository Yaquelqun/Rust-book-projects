use crate::structs::Request;

pub fn interpret(request: &mut Request) -> Result<String, u8> {
    match request.parsed[0].as_str() {
        "ADD" => {
            println!("We are adding stuff");
            let pokemon_list = request.pokedex.entry(String::from(&request.parsed[2])).or_insert(Vec::new());
            pokemon_list.push(request.parsed[1].clone());
            // println!("Pokemon {} added to {} type", String::from(&request.parsed[1]), String::from(&request.parsed[2]));
            return Result::Ok(format!("Pokemon {} added to {} type", request.parsed[1], request.parsed[2]));
        },
        "LIST" => {
            let mut output = String::new();
            // println!("{:?}", request.pokedex.get(&request.parsed[2]).unwrap() );
            for pokemon in request.pokedex.get(&request.parsed[2]).unwrap() {
                let new_line = pokemon.clone() + &String::from(", ");
                output.push_str(&new_line);
            };
            // println!("{:?}", output );
            output.pop();
            output.pop();
            return Result::Ok(output);
        },
        "QUIT" => return Result::Err(0),
        _ => return Result::Err(0)
    }
}