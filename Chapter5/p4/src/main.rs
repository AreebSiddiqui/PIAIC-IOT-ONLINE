#[derive(Debug)]
struct Book {
    name:String,
    author: String,
    price: u16,
    availiblity: bool,
}

fn main() {
     let book_1 = Book {
        name: String::from("Book A"),
        author: String::from("Author A"),
        price: 500,
        availiblity: true,
    };


    let book_2 = Book {
        name:String::from("Book B"),
        author:String::from("Author B"),
        ..book_1
    };
    
println!("{:#?}",book_2);
}

