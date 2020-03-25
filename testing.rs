pub fn main(){
    let mut x = 5;
    let y = "Cookies";  
    println!("These {} cookies are my {}", x, y);
    x = eat_cookie(x);
    println!("These {} cookies are my {}", x, y);
}

pub fn eat_cookie(x: i32) -> i32{
    println!("I ATE A COOKIE");
    return x - 1; 
}