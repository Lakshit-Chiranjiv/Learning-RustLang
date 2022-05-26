
enum State{
    Activated,
    Deactivated,
    Pending
}

fn check_state(st: State){
    if matches!(st,State::Activated) {
        println!("Activated state");
    }
    else if matches!(st,State::Deactivated) {
        println!("Deactivated state");
    }
    else {
        println!("Pending state");
    }
}

pub fn run(){
    let s1 = State::Activated;
    let s2 = State::Deactivated;
    let s3 = State::Pending;
    
    check_state(s1);
    check_state(s2);
    check_state(s3);
    
}