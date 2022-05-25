//normal struct
struct Student {
    name: String,
    roll: i32,
    sex: char,
    present: bool
}

//tuple struct
struct Order(String,i32);

//struct with function
struct Book {
    name: String,
    pages: i32,
    price: f64,
    sold: bool
}

impl Book {
    fn create_book(nm: &str,pg: i32,prc: f64,sld: bool) -> Book {
        Book {
            name: nm.to_string(),
            pages: pg,
            price: prc,
            sold: sld
        }
    }
    
    fn change_price(&mut self,price: f64) {
        self.price = price;
    }
    
    fn get_total_price_with_tax(&self) -> f64 {
        let tax = 0.1 * self.price;
        self.price+tax
    }
    
    fn get_book_details(&self) -> String {
        format!("{} consists of {} pages and costs ₹{}.",self.name,self.pages,self.price)
    }
    
}

pub fn run(){
    let mut s1 = Student {
        name: "Harry".to_string(),
        roll: 435,
        sex: 'M',
        present: true
    };
    
    println!("s1 = {},{},{},{}",s1.name,s1.roll,s1.sex,s1.present);
    s1.roll = 438;
    println!("s1 = {},{},{},{}",s1.name,s1.roll,s1.sex,s1.present);
    
    let mut o1 = Order("Shoes".to_string(),77);
    println!("o1 = {} - {}",o1.0,o1.1);
    o1.1 = 32;
    println!("o1 = {} - {}",o1.0,o1.1);
    
    let mut b1 = Book::create_book("Immortals of Meluha",723,1500.44,false);
    println!("b1 = {},{},{},{}",b1.name,b1.pages,b1.price,b1.sold);
    println!("b1 old price with tax = {}",b1.get_total_price_with_tax());
    b1.change_price(1600.55);
    println!("b1 old price with tax = {}",b1.get_total_price_with_tax());
    println!("Book details : {}",b1.get_book_details());
    
}