#[derive(Debug)]
struct Book {
    name: String,
    author: String,
    price: u16,
    availbility: bool,
} //tempelate



fn main() {
    let mut book_1 = Book {
        name:String::from("Book A"),
        price: 500,
        availbility:true,
        author: String::from("Author A"),
    };
book_1.name = String::from("Book AA");
    

    println!("{:#?}",book_1);
    

}
