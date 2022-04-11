pub fn run(){
    println!("Variables in Rust");
  
    let name = "Lakshit";
    let mut age = 20;
  
    const EXPERIENCE: i32 = 8;
    println!("name : {}\nage: {}\nexperience: {}",name,age,EXPERIENCE);
    age = 21;
  
    println!("name : {}\nage: {}\nexperience: {}",name,age,EXPERIENCE);
  
  
    let (a,b,c) = (10,20,30);
    println!("a:{}",a);
    println!("b:{}",b);
    println!("c:{}",c);