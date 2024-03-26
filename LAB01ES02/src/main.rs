use std::{fs, time::SystemTime};

enum Error{
    Simple(SystemTime),
    Complex(SystemTime, String)
}

pub enum MulErr{
    Overflow,
    NegativeNumber

}
pub fn  mul(a:i32,b:i32) -> Result<u32,MulErr>{
    if a<0 || b<0{
           return  Err(MulErr::NegativeNumber);
    }
    let i:i64=(a as i64)*(b as i64);
    if i>0xFFFFFFFF{
            return Err(MulErr::Overflow);
    }
        return Ok((a*b) as u32);
}
fn print_mul(x:Result<u32,MulErr>){
    match x{
        Result::Ok(c) =>{
            println!("Il risultato è: {}",c);
        },
        Result::Err(msg)=>{
            match msg{
                MulErr::NegativeNumber =>{println!("Numero negativo!")},
                MulErr::Overflow=>{println!("Overflow!")}
            }

        }
    }

}
fn print_error(e: Error){
    match e{
        Error::Simple(time) => {
            println!("Errore semplice! {:?}",time);
        },
        Error::Complex(time,str) =>{println!("{:?}", str)}

    }

}

struct Node {
    name: String,
    size: u32,
    count: u32,
}

impl Node {
    pub fn new(nome:String) -> Node {
        Node {name:nome,size: 0,count: 0}
    }
    pub fn size(mut self,i:u32) -> Node{
        self.size=i;
        self
    }
    pub fn count(mut self,i:u32) -> Node{
        self.count=i;
        self
    }
    pub fn to_string(&self){
        println!("name:{} size:{} count:{}",self.name,self.size,self.count);
    }
    pub fn grow(&mut self) {
        self.size+=1;
        
    }
    pub fn inc(&mut self) {
        self.count+=1;
        
    }
}
    
fn main() {
    //read_to_string
    //let s1=fs::read_to_string("../test.txt");
    // s1 é una tupla, se file non esiste mi da errore
    //println!("{:?}",s1);

    //read
    //let s2=fs::read("../test.txt");
    //println!("{:?}",s2);
    //ENUM
    let errore_semplice=Error::Simple(SystemTime::now());
    let s="A complex error has occurred!";
    let errore_complesso=Error::Complex(SystemTime::now(),s.to_string());
    print_error(errore_semplice);
    print_error(errore_complesso);
    print_mul(mul(10,10));
    //STRUCT
    let mut _node= Node::new("Nodo giovanni".to_string()).size(10).count(5);
    _node.to_string();
    _node.grow();
    _node.inc();
    _node.to_string();



}
