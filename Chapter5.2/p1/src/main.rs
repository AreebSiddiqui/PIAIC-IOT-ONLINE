fn main() {
    let rec1 = (100,50);
    println!("The area of sqaure is: {}",area(rec1));

}

fn area (dimensions : (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}