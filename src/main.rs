fn main(){
   println!("Hello World");  
   println!("{}",fibonacci(8))
}

fn fibonacci(num: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;
    
    if num == 0{
        return 1
    }
    if num == 2 {
        return 2
    }

    for i in 0..(num - 1) {
        let temp = second;
        println!("{} is {}", i, second);
        second = second + first;
        first = temp;
        
    
    }
    return second;
}