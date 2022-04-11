
pub fn run(){
    //simple printing
    println!("printing");
  
    //formatting
    println!("some number {}",7);
  
    println!("{} is a superbb {}!!","Lakshit","Coder");
  
    //positional arguments formatting
    println!("{0} is my {1},{2} is the {3} of {0}","India","country","Delhi","capital");
  
    //named arguments
    println!("{name} was {doing} at {time}.",name="Lakshit",doing="coding",time="8AM");
  
    //placeholder traits
    println!("binary for 10 : {:b}",10);
    println!("octal for 10 : {:o}",10);
    println!("hexadecimal for 10 : {:x}",10);
  
    //debug trait
    println!("{:?}",(34,73,244,"Lakshit","hello",true));
  
    println!("10 + 15 = {}",(10+15));
    
  }