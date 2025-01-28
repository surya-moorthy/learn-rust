
enum Operator {
    Addition(u32,u32),
    Subtraction(u32 , u32),
    Multiplication(u32, u32),
    Division(u32, u32),
  }
  
  fn main() {
  
    let assign = Operator::Subtraction(6,5);
    println!("result : {}",operation(assign))
  
  }
  
  fn operation(operator : Operator) -> u32{
      match operator {
        Operator::Subtraction(a,b) => a - b,
        Operator::Addition(a,b ) => a + b,
        Operator::Multiplication(a,b ) => a * b,
        Operator::Division(a,b ) => a / b,
      } 
  }
  
  
  