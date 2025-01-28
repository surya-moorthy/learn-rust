fn main() {
    let ans = is_even(21);
    println!("{}",is_even(32));
    println!("{}",ans)
}

fn is_even(num: u32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}