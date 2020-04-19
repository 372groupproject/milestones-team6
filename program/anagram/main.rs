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

    //Pass in the two strings as lowercase string types. 
    anagram(&mut input1.to_lowercase(), &mut input2.to_lowercase());
    
}

pub fn anagram(input: &mut String, input2: &mut String){
   
    //Replace all spaces with null in strings
    let s = input.replace(" ", "");
    let x = input2.replace(" ", "");
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