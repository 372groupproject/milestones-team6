
mod uaccess {
    pub mod sign_up {
        pub fn add_to_class() {println!("Added Class");}

        pub fn add_to_section() {println!("Added section");}
    }

}

pub fn add_classes(){
    add_to_class();
}


mod student_classes {
    #[derive(Debug)]
    pub enum Classes {
        Automata,
        Comparative,
    }
}


use crate::uaccess::sign_up::add_to_class;

use crate::student_classes::Classes;

pub fn main() {
    // Absolute path
    crate::uaccess::sign_up::add_to_class();

    // Relative path
    uaccess::sign_up::add_to_class();
    uaccess::sign_up::add_to_section();

    //Use 
    add_classes();

    let class1 = Classes::Automata;
    let class2 = Classes::Comparative;

    println!("{:?}", class1);
    println!("{:?}", class2);


}


