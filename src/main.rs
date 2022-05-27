use std::io;
use std::io::Write;
mod cypher;

fn main() {
    let mut user_input = String::new();
    
    println!("Enter message to apply rot16 cipher too!");
    print!("message : ");

    // flush stdout so message prints w/o newline before reading
    io::stdout().flush().unwrap();
    
    // get user input
    io::stdin()
        .read_line(&mut user_input)
        .expect("Could not read user input...");

    // apply cipher to user input and return message
    let cypher_output: String = cypher::rot13(& user_input);

    // print results
    println!("{}", cypher_output);
}
