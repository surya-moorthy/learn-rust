// we can make it public by adding "pub" infront od trait
trait Summary {
    fn summarize(&self) -> String;
    fn greetinngs(&self) -> String;
  //we can declare and implement at the trait itself 
    fn hithere(&self) -> String {
      return format!("Hello there");
   }
  }
  
  struct User {
    name: String,
    age: u32,
  }
  
  impl Summary for User {
  
    //we can create our own custom functions here
      fn summarize(&self) -> String {
          return format!("The name is {} and my age is {}",self.name,self.age);
      }
      fn greetinngs(&self) -> String {
          return format!("Hello my name is {} ",self.name);
      }
  }
  
  //traits as parameters 
  
  fn implement_fn(lala: &impl Summary) {
    println!("{:?}",lala.summarize());
    println!("{:?}",lala.greetinngs());
  }
  
  fn main() {
      let user = User {
        name: "surya".to_string(),
        age: 19
      };
      implement_fn(&user);
     
  }
  
  