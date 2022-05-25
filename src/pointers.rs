pub fn run() {
    
    //array is primitive so pointer reference wasn't needed
    let a = [2,4,1];
    let b = a;
    
    println!("a = {:?}",a);
    println!("b = {:?}",b);
    
    //vectors are non-primitive so pointer reference will be needed
    let c = vec![4,2,6];
    let d = &c;
    println!("c = {:?}",c);
    println!("d = {:?}",d);
    
    let x = 66;
    let y = x;
    let z = &x;
    println!("x = {}",x);
    println!("y = {}",y);
    println!("z = {}",*z);
    println!("z = {}",z);
    
}