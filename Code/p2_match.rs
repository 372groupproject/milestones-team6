enum Class {
    Comparative, 
    Automata,
    WebDev,
    Databases 
}
fn value_in_class(class: Class) -> u32 {
    match class {
        Class::Comparative => 372,
        Class::Automata => 473,
        Class::WebDev => 337,
        Class::Databases => 460,
    }
}
pub fn main(){
   let com = value_in_class(Class::Comparative);
   let auto = value_in_class(Class::Automata);
   let web = value_in_class(Class::WebDev);
   let data = value_in_class(Class::Databases);
   println!("Class is {}", com);
}