fn main() {
    let string = "Hello".to_string();
    let index_of_letter = find_first_char('e',string);
    match index_of_letter {
       Some(value) => println!("{}",value),
       None => println!("character not found")
    }
 }
 
 fn find_first_char(chr: char, str: String) -> Option<i32>{
 
   for (index , char) in str.chars().enumerate() {
     if char == chr {
       return Some(index as i32)
     }
   } 
 
     return None;
  
 }