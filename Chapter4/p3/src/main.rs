fn main() {
let s = String::from("Hello"); // s comes into scope 
take_ownership(s); // s is moved into take_ownership
//println!("{}",s);

let num = 5;    // num comes into scope
makes_copy(5);  // num is copied to the function makes_copy
println!("From the main function: {}",num);

}
fn take_ownership(x: String) { // x comes into scope now
println!("{}",x); 
} //x is out of sope and will dropped


fn makes_copy(x:i32) {
println!("{}",x);
}