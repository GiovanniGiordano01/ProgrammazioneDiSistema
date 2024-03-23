use std::fs;

enum Error{
    Simple(SystemTime);
    Complex(SystemTime, String); 
}
impl Error{

    
}

fn main() {
    //read_to_string
    let s1=fs::read_to_string("../test.txt");
    // s1 é una tupla, se file non esiste mi da errore
    println!("{:?}",s1);

    //read
    let s2=fs::read("../test.txt");
    println!("{:?}",s2);

}
