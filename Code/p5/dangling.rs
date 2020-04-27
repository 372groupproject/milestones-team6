
pub fn main(){
    let string = give();
}

fn give() -> &String {
    //Reference goes out of scope after give finishes running
    let string = String::from("csc372");
    println!("{}", string);
    return &string;
}