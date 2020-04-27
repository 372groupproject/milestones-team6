pub fn main(){
    //Race condition
    let mut string = String::from("Hello");
    let string1 = &mut string;
    let string2 = &mut string;
    println!("{} {}", string1, string2);
}