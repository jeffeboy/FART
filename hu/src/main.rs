use std::vec;

fn main() {
	let mut ni = 0;
	let mut xt = 0;
	let mut yt = 0;
    for x in 0..1000 {
   		for y in 0..1000 {
   			if (check_palindrome(x*y) != false) && (x*y as i32 >ni) {
   			ni = x*y;
   			xt=x;
   			yt=y;
    		}
    	}
    }
    println!("{} {} {}",xt,yt,ni)
}

fn count(a:i32) -> (Vec<i32>,i32){
    let b = a.to_string().chars().count() as i32;
    let mut num = vec![];
    
    for c in a.to_string().chars() {
    	num.push(c.to_digit(10).unwrap() as i32)	
    }
    return (num,b-1)
    
}    

fn check_palindrome(namba:i32) -> bool { 
	let (a, b) = count(namba);
	let mut s = 0;
	for i in 0..b {
		if a[i as usize]==a[(b-i) as usize] {
			s+=1
		}
	}
	return s==b
}
	
