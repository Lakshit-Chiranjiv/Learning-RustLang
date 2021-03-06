use std::mem;
pub fn run(){
    let mut nums: Vec<i32> = vec![1,4,2,5,2,6,5];
    let mut names: Vec<&str> = vec!["Lakshit","Hannah","Clay","Meg"];
    
    println!("{:?}",nums);
    println!("{:?}",names);
    
    //length
    println!("nums length : {}",nums.len());
    println!("names length : {}",names.len());
    
    //push
    nums.push(41);
    nums.push(88);
    names.push("Sophie");
    println!("{:?}",nums);
    println!("{:?}",names);
    
    //pop
    nums.pop();
    names.pop();
    println!("{:?}",nums);
    println!("{:?}",names);
    
    //accessing index values
    println!("nums index 2 : {}",nums[2]);
    println!("names index 1 : {}",names[1]);
    
    //reassign index values
    nums[2] = 14;
    names[1] = "Hannah Baker";
    println!("{:?}",nums);
    println!("{:?}",names);
    
    //size taken in memory
    println!("nums takes {} bytes",std::mem::size_of_val(&nums));
    println!("names takes {} bytes",mem::size_of_val(&names));
    
    //slicing
    let num_slice1: &[i32] = &nums;
    let num_slice2: &[i32] = &nums[1..5];
    let names_slice: &[&str] = &names[1..3];
    println!("{:?}",num_slice1);
    println!("{:?}",num_slice2);
    println!("{:?}",names_slice);
    
    //looping
    println!("Printing nums values");
    for i in nums.iter(){
        println!("num -> {}",i);
    }
    println!("Printing names values");
    for i in names.iter(){
        println!("name -> {}",i);
    }
    
    //looping & mutating
    for i in nums.iter_mut(){
        *i += 5;
    }
    println!("{:?}",nums);
}