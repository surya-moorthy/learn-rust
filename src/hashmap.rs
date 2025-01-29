use std::collections::HashMap;

fn main() {
    let vec:Vec<(String,i32)> = vec![("string1".to_string(),32),("String2".to_string(),23)];
    let ans = get_hashmap_as_tuples(vec);
    println!("{:?}",ans);
  
}

fn get_hashmap_as_tuples(pairs: Vec<(String, i32)>) -> HashMap<String,i32> {
   let mut hm = HashMap::new();
   for (data , value) in pairs {
    hm.insert(data, value);
   } 
   return hm;
}