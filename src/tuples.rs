pub fn run(){
    //group of elements of different types
    let game: (&str,i32,f64,&str) = ("Cricket",11,98.76,"world");
    println!("{} is played with {} players in each team and this game is loved by {}% of people around the {}.",game.0,game.1,game.2,game.3);
}