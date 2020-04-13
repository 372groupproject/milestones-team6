
pub fn main(){
    let mut tup = (1,2,'c');
    let mut tup1 = ((1,2,'b'),(1,2,'a'));
    tup.2 = 'w';
    tup1.0 = tup;
    (tup1.1).1 = 4;
    println!("tup is {:?}", tup);
    println!("tup1 is {:?}", tup1);
}