use std::io;

fn main(){
    println!("Hi there!");
    let mut input1 = String::new();
    println!("String one!");
    //Prompts user for first string and then checks if it is an error or not
    match io::stdin().read_line(&mut input1) {
        Ok(_) => {
            //arr1 = anagram(&mut input1);
        },
        Err(e) => println!("Something went wrong {}", e)
    }
    let mut input2 = String::new();
    println!("String two!");
    //Prompts user for second string and then checks if it is an error or not
    match io::stdin().read_line(&mut input2) {
        Ok(_) => {
            //arr2 = anagram(&mut input2);
        },
        Err(e) => println!("Something went wrong {}", e)
    }

    //Pass in the two strings as string types. 
    anagram(input1,input2);

    // INVALID PRINT
    // println!("are {} and {} anagrams?", input1, input2);
    //Call to anagram tranfered ownership to anagram. Can be fixed by changing anagram to
    //anagram(input: &mut String, input2: &mut String)
    //OR
    //(input: String, input2: String) -> (String, String) we can return both inputs back to main and then main
    //could print them
    
    
}

pub fn anagram(input: String, input2: String){
   
    //Replace all spaces with null in strings
    let s = input.to_lowercase().replace(" ", "");
    let x = input2.to_lowercase().replace(" ", "");
    //Convert the strings into arrays of their byte values
    let mut arr = s.into_bytes();
    let mut arr2 = x.into_bytes();
    //Sort the two lists based on their byte values
    arr.sort();
    arr2.sort();
    //If the two arrays are equal they are anagrams
    if arr == arr2 {
        println!("true");
    }else {
        println!("false");
    }
    
}