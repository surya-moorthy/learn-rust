//generics 
fn main() {
    let greater_value  = simple_func(1,2);
    let matched_string = simple_func("hello".to_string(),"heelo".to_string());
    println!("{:?}",greater_value);
    println!("{:?}",matched_string);
}
fn simple_func<T: std::cmp::PartialOrd>(a: T , b: T) -> T {
 if a == b {
  return a; 
 }
 return b;
}