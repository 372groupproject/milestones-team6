

fn main(){
    //First string literal
    println!("String 1 is: ");
    let string1 = "Hello world";
    println!("{}", string1);

    //Second string literal
    println!("String 2 is: ");
    let string2 = "lwdor    Hello";
    println!("{}", string2);

    //Pass them into anagram2
    anagram2(string1, string2);
    //VALID PRINT
    println!("are '{}' and '{}' anagrams?", string1, string2);
    

}

//Anagram2 takes in 2 string literals, because their size must be known at compile time we must pass them in
//as pointers to the values of string1 and 2. The strings are known as read-only not stored on stack or heap
//The pointers are stored on the stack. When we pass in the string a new pointer is copied from the original string1 and 
//added to the stack.
pub fn anagram2(string1: &str, string2: &str){

    //Because string literals are immutable we add each char to a vector
    //We then use the retain method to only keep chars that are NOT a space
    let mut chars: Vec<char> = string1.chars().collect();
    chars.sort();
    let space = ' ';
    chars.retain(|&x| x != space);

    let mut chars1: Vec<char> = string2.chars().collect();
    chars1.sort();
    chars1.retain(|&x| x != space);

    //Print out each list sorted
    println!("Sorted: ");
    println!("{:?}", chars);
    println!("{:?}", chars1);

    //Print out if they are sorted or not
    if chars == chars1 {
        println!("True");
    }else{
        println!("False");
    }
}