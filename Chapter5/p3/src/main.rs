#[derive(Debug)]
struct Book {
    name:String,
    author: String,
    price: u16,
    availiblity: bool,
}

fn main() {
    let book_1 = build(String::from("Book A"), String::from("Author A"));
    println!("{:#?}",book_1);
}

fn build_book (name:String, author:String) -> Book {
    Book{   
        name,
        author,
        price:500,
        availiblity:true,
    }
}