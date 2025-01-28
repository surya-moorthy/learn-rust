use chrono::{Local, Utc};

fn main() {
  let time = Local::now();
  let u_time = Utc::now();  // here Local is a struct that has now function
  println!("my local time is {}",time);
  println!("Univeraal time is {}",u_time);
}