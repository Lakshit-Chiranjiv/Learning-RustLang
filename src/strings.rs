pub fn run(){
    //Primitive Immutable String
    let ps1 = "This is a string";

    println!("ps1 : {}",ps1);
    //length
    println!("Length : {}",ps1.len());
    
    //empty check
    println!("ps1 is empty : {}",ps1.is_empty());
    
    //contains
    println!("ps1 contains string : {}",ps1.contains("string"));
    
    //replace
    println!("replacing stringx to strings in ps1 : {}",ps1.replace("string","strings"));
    
    //looping on words
    for word in ps1.split_whitespace(){
        println!("{}",word);
    }

    
    //---------------------------
    //Growable Strings
    let mut gs1 = String::from("Now this one is a growable string");
    
    println!("gs1 : {}",gs1);
    //length
    println!("Length : {}",gs1.len());
    
    //capacity
    println!("Capacity : {}",gs1.capacity());
    
    //push character
    gs1.push('x');
    println!("gs1 : {}",gs1);

    //push string
    gs1.push_str(" added");
    println!("gs1 : {}",gs1);
    
    //empty check
    println!("gs1 is empty : {}",gs1.is_empty());
    
    //contains
    println!("gs1 contains stg : {}",gs1.contains("stg"));
    
    //replace
    println!("replacing stringx to strings in gs1 : {}",gs1.replace("stringx","strings"));
    
    //looping on words
    for word in gs1.split_whitespace(){
        println!("{}",word);
    }
    
    //creating string with capacity
    let mut gs2 = String::with_capacity(10);
    println!("gs2 capacity : {}",gs2.capacity());
    gs2.push('h');
    gs2.push('e');
    gs2.push_str("llo");
    println!("gs2 : {}",gs2)

}