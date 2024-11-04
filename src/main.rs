mod token;

use token::tokenize;

fn main() {
    match tokenize("") {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
