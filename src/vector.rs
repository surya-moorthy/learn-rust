// fn main() {

//     let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(2);
//     println!("{:?}",vec)
//   }
  

fn main() {
    let mut vec1 = Vec::new();
    vec1.push(1);
    vec1.push(2);
    vec1.push(3);
    let mut vec2 = Vec::new();
    vec2.push(4);
    vec2.push(5);
    vec2.push(6);
    
    let mut vec = vec![1,2,3,4];
    vec.push(4);
    //the type is automatically infered
    println!("{:?}",vec);
  
    // let ans = even(&vec);

    // println!("{:?}",ans);
    let ans = even(&mut vec2);
    even_bet_app(&mut vec1);
    println!("{:?}",vec1);
    println!("{:?}",ans);
}

fn even(vec: &Vec<i32>) -> Vec<i32>{
  let  mut new_vec=Vec::new();
  for val in vec {
    if val % 2 == 0 {
      new_vec.push(*val)
    }

  }
  return new_vec;
}

fn even_bet_app(vec : &mut Vec<i32>) {
   let mut i = 0;
   while i < vec.len() {
    if vec[i] % 2 != 0 {
       vec.remove(i);
    }
    i += 1
   } 
}
