use std::vec;
mod fart3dot1;

fn main() {
	let mut a = 0;
	let mut b = 0;
	let mut i = 0;
	while b<500 {
		i += 1;
		a = fart3dot1::nat_1(a, i);
		b = fart3dot1::cdev(a).len();
		println!("a is {} b is {}",a,b);
	}
	println!("a = {}, len = {}", a, b);
}

fn iv(a:Vec<i32>) { // Iterates over vector
	for &i in &a {
		println!("{}",i);
	}
}
	
