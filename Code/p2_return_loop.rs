pub fn main() {
    let mut class = 370;

    let result = loop {
        class += 1;

        if class == 372 {
            break class;
        }
    };

    println!("The class is {}", result);
}