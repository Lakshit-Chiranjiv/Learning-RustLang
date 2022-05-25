pub fn run(){
    say_hello_to("Jerry");
    
    println!("sum of 5 and 6 is {}",add(5,6));
    let a = add(3,9);
    println!("a = {}",a);
    
    //closure functions (allows to use values from outside the function scope)
    let mult_add12 = |x: i32, y: i32| x*y+a;
    let b = mult_add12(4,5);
    println!("b = {}",b);
    
}

//simple function
fn say_hello_to(person: &str) {
    println!("Hello {}",person);
} 

//function with return type
fn add(m: i32, n: i32) -> i32 {
    m+n
}