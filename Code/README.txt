Main.rs prints out hello world and then calls another rust program testing.rs.
The goal of the second program was to show how we can use other rust programs in our main using mod. 
And then showing how to pass variables and then print out changes.

To run the code it needs to be in the same file and the use commands cargo build and cargo run to compile and run the program.

Not entirely sure what to put in the Makefile for a rust program. Or for a Makefile in general 
hopefully the cargo run and build commands are enough but I doubt it. 

To run rust code we have visual code studio with a rust extension. When running code we use the cmd terminal.
To compile and run code wrtie
cargo build
cargo run

For milestone 2 we have included a new main.rs that has used mod to import each of our code snipets. Currently
all of the code snippets except for the p2_if are commented out. This was to avoid an error with our panic! example
and the infinite loop example which can be quit by hitting ctrl_c. To see each example run on its own simply comment
out the snippet just run and uncomment the snippet you wish to run. Then do cargo run and it should be fine.

To make running the snippets easier i have included a p2_snippets folder that has all the needed information to run
in visual code studio using cargo run and cargo build.

I added a folder called p2 that holds the example program for milestone 2.
