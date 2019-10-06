#[derive(Debug)]
struct Rectangle {
    height:u32,
    widht:u32,
}
fn main () 
{
    let rec_1 = Rectangle {
        height: 100,
        widht: 50,
    };

println!("The area of square is {}",area(&rec_1));

println!("{:#?}",rec_1);

}


fn area (rec: &Rectangle) -> u32 {
    rec.height*rec.widht
}