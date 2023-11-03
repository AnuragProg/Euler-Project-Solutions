use crate::util::{next_prime, is_palindrome};

pub fn p1() -> i64{
    let mut res: i64 = 0;
    for i in 1..1000{
        if i%3==0 || i%5==0{ res += i }
    }
    res
}


pub fn p2() -> i32{
    let (mut first, mut second)= (1_i32,2_i32);
    let mut sum = 0_i32;
    while second <= 4e6 as i32{

        if second%2==0{
            sum += second;
        }

        let temp = first;
        first = second;
        second += temp;
    }
    sum
}


pub fn p3() -> i64{
    let mut num = 600851475143_i64;
    let mut prime = 1_i64;
    
    let mut fail_safe = 1e6 as i64;
    while num != 1 && fail_safe != 0{

        prime = next_prime(prime);
        //println!("current prime {}", prime);
        fail_safe -= 1;

        while num!=1 && num%prime==0{
            num /= prime;
        }
    }
    prime
}


pub fn p4() -> i64{
    let (start,end) = (100,999);
    //let (start,end) = (10,99);

    let mut max = 0_i64;
    let mut temp;
    for i in (start..=end).rev(){
        for j in (start..=end).rev(){
            temp = i*j;
            if is_palindrome(temp) && max<temp{
                max = temp;
            }
        }
    }
    max
}

pub fn p5()->i64{
    let mut num = 21;

    'outer: loop {
        for i in 2..20{
            if num%i != 0{
                num += 1;
                continue 'outer;
            }
        } 
        return num;
    }
}


pub fn p6()->i64{
    let (sum_of_squares,sum) = (1..=100).fold((0,0), |mut acc,val|{
        acc.0 += val*val;
        acc.1 += val;
        acc
    });
    println!("{}, {}", sum, sum_of_squares);
    (sum*sum) - sum_of_squares

}
