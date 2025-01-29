fn main() {
    let longest ;
    let string1 = String::from("Hello");
   {
     let string2 = String::from("HelloWorld");
    longest = longest_string(&string1, &string2);
   }
    println!("{:?}",longest);
  } 
  
  fn longest_string<'a>(a: &'a str,b: &'a str) -> &'a str {
  
    // error : missing lifetime specifier
    // this function's return type contains a borrowed value, but the signature does not say 
    // whether it is borrowed from `a` or `b`
    // when we try to borrow reference of b in case of &str rust , asks 'what is the lifetime of b ', means 
    // when string2 reaches out of scope then 'b' in function will also get out of scope.
    
     //we can solve this by using generic annotations
     //'a describe a relationships between input and output args
    if a.len() > b.len() {
      return a;
    }
   return b;
  
  }
  
  struct User<'a> {
    name: &'a str,
  }
  
  fn main() {
   let name = String::from("surya");
   let user = User {
      name: &name
   };
   println!("{:?}",user.name);
  }