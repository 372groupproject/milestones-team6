

pub fn main(){

    let mut string = String::from("Hello");

    //Gives ownership to give_take
    string = give_take(string);

    println!("{}", string);

}

fn give_take(string: String) -> String{

    let mut string1 = string;
    string1.push_str(" World");
    
    println!("{}", string1);
    //We return ownership of string1 which is the current owner of "Hello World" to main
    return string1;
}