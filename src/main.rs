use token::tokenize;

mod token;

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
