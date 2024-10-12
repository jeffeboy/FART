pub fn cdev(a:i32) -> Vec<i32> {
    let mut c = a;
    let mut d = 0;
    let mut DA:Vec<i32> = vec![];
    let mut m:bool = false;
    let mut mm:bool = false;
    while m != true {
        d+=1;
        if a%d==0 {
            DA.push(d);
            
            if mm == false && d != 1 {
                c = a/d;
                mm = true;
            }
        }
        if c<=d {
            m = true;
        }
    }
    if !DA.contains(&d) {
        DA.push(a);
    }
    return DA 
}

pub fn nat_1(a:i32,i:i32) -> i32 {
    return a+i
}