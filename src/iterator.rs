//Iterator

fn main() {
    let v1 = vec![1,2,3,4];
    let v2 = vec![5,6,7,8];
    let mut v3 = vec![9,10,11];
    v3.push(12);
  
    //using for loops 
    println!("iterate list of items using for loo");
    for val in v1 {
         print!("{:?}",val);
    }
    //using iter()
    //converts collection into a iterator by borrowing them
    println!("Iterate using iter()");
    let v2_iter = v2.iter();
    for val in v2_iter {
      println!("{:?}",val);
    }
   
   //iterate using iter_mut() for having mutable iterator
   
   let v3_iter = v3.iter_mut();
   for val in v3_iter {
    println!("{:?}",val);
   }
  
  
   //iterate using .next() with while which introduces Option type to each value in the vector oor any collection 
   let first_number_v3 = v3.iter().next();
   println!("{:?}",first_number_v3);
  
   //using into_iter()
     let vec1 = vec![1,2,3];
     let iter = vec1.into_iter();
  
     for val in iter {
         println!("{:?}",val);
     }
    //  println!("{:?}",vec1);
    // error  : borrow of moved value: `vec1`
  
   }
  