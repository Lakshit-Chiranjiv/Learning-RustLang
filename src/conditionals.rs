pub fn run(){
    let a = 16;
    let b = 82;
    
    //if else
    if a>b {
        println!("a is greater");
    }
    else {
        println!("b is greater");
    }
    
    let c = 20;
    let d = 42;
    
    //if else-if else ,&&
    if c>=a && c>=b {
        println!("c is greatest");
    }
    else if b>=c && b>=a {
        println!("b is greatest");
    }
    else{
        println!("a is greatest");
    }

    //if else-if else ,&&, ||
    if c>=a && c>=b || d==42 {
        println!("c is greatest");
    }
    else if b>=c && b>=a || d==34 {
        println!("b is greatest");
    }
    else{
        println!("a is greatest");
    }
    
    //shorthand if else
    let msg = if d>50 { "d is more than 50" } else { "d is less than or equal to 50" };
    println!("{}",msg);
}