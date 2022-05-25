pub fn run() {
    let mut val = 20;
    
    //infinite loop with break
    loop {
        println!("val : {}",val);
        val += 1;
        if val == 31 {
            break;
        }
    }
    
    //while loop
    while val < 45 {
        if val % 2 == 0 {
            println!("val {} is even",val);
        }
        else {
            println!("val {} is odd",val)
        }
        val += 1;
    }
    
    //for range loop
    for i in 1..10 {
        println!("i value is {}",i);
    }
}