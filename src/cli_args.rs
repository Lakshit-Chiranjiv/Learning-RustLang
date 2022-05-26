use std::env;
pub fn run() {
    let args: Vec<String> = env::args().collect();
    let msg = args[1];
    println!("{:?}",args);
    println!("msg = {}",msg);
}