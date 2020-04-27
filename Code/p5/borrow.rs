pub fn main(){

    let string = String::from("Hello");

    //borrow takes reference to string main still owns it
    borrow(&string);

    println!("{}", string);

}

fn borrow(string: &String){
    
    println!("{}", string);
    //pass reference to borrow2 main still owns it
    borrow2(&string);

}

fn borrow2(string: &String){
    
    println!("{}", string);

}