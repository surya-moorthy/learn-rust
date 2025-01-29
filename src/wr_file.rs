use std::fs::read_to_string;

fn main() {
    let result = read_to_string("hello.txt");

    match result {
         Ok(data) => println!("{:?}",data),
         Err(err) => println!("Failed to real file , the error: {:?}",err),
    }
}  