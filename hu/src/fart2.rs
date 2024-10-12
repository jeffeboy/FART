pub fn sqsum(n:i32) -> i32 {
    let mut s = 0;
    for i in 0..n+1 {
        s += i*i;
    }
    let mut b = 0;
    for i in 0..n+1 {
        b+=i;
    }
    println!("s is {}, b is {}, difference is {}",s,b,s-b);
    return s-b
}