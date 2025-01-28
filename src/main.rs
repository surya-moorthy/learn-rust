fn main(){
  let my_string = String::from("Hello World"); 
  let  _l_str = "Ten".to_string();
  println!("{}",str_count(_l_str));
  println!("{}",str_count(my_string));
  let user = User {
      first_name: "surya".to_string(),
      last_name: "moorthy".to_string(),
      age : 32,
  };
  println!("{},{},{}",user.first_name,user.last_name,user.age)
}

struct User{
  first_name: String,
  last_name: String,
  age: i32,
}


fn str_count(str: String) -> usize {
  return str.chars().count();
} 

