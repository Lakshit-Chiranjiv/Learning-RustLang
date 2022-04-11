pub fn run(){

    println!("Data types in Rust");
  
    //u8,i8,u16,i16,u32,i32,u64,i64,u128,i128
    //f32,f64
    //bool,char
    let a = 7;//default i32
  
    let b = 72.32;//default f64
  
    let c: i8 = 5;
    let d: f32 = 23.21;
  
    println!("max i32 : {}",std::i32::MAX); 
    println!("min i32 : {}",std::i32::MIN); 
  
    println!("max f32 : {}",std::f32::MAX); 
    println!("min f32 : {}",std::f32::MIN); 
  
    let active = false;
  
    let wrong: bool = true;
  
    let big: bool = 15 > 2;
  
    let p = 'p';
    let q: char = 'q';
    let r: char = '\t';
    let s = '🔥';

    //unitialized variables give errors
  
    println!("{:?}",(a,b,c,d,active,wrong,big,p,q,r,s));
  }