
pub fn is_prime(num: i64) -> bool{
    for i in 2..=((num as f64).sqrt().floor() as i64){
        if num%i==0{
            return false
        }
    }
    true
}


pub fn next_prime(mut cur: i64) -> i64{
    loop {
        cur += 1;
        if is_prime(cur){
            return cur
        }
    }
}

pub fn is_palindrome(num: i64) -> bool{
    let num_str = num.to_string();
    let num_str_rev : String = num_str.chars().rev().collect();
    num_str == num_str_rev
}
