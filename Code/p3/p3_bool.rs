

pub fn main(){
    let mut x = true;
    let y = 0;

    
    let z = y || x;
    x = y && x;
    let a = !x;
    println!("z is {} x is {} a is {}",z, x, a);
}