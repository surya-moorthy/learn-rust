
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

// the below and above uses code a concept called iterators , in below it uses .iter() to have reference on iterating the vector 
// fn main() {
//   let vec = vec![1,2,3,4];
//   let new_vec = fiter_out_map(vec);
//   println!("new vector:{:?}",new_vec);
// }

// fn fiter_out_map(vec: Vec<i32>) -> Vec<i32> {
//   let iter = vec.iter().filter(|x| *x % 2 == 1).map(|x| x *2);
//   let new_vec:Vec<i32> = iter.collect();
//   //changes into an iterate to vector
//   return new_vec;
// }