The first program is anagrams implemented with string types. It is fully functional. The second
program will be implemented with string literals so that we can highlight how ownership affects the
two. Anagram has been changed and we have added some comments showing the multiple ways that ownership
can change the way Anagrams is run using string types. The current Anagram gives the function Anagram 
ownership of variables where as before using &mut String, instead of the current String, Main simply
let the anagram function borrow input1 and input2. You can also do a give and take where Main gives
anagram ownership and then it can return ownership back to main by returning the variables passed in.

To run anagram we have been using Visual studio Code with a rust extension. With our anagram folder go
to file and add folder to workspace add our anagram file to the workspace. Then anagram should show up
in your workspace left click on anagram and select open in terminal to get the terminal for that project
Then just type cargo build to compile and the cargo run to run the program. 

