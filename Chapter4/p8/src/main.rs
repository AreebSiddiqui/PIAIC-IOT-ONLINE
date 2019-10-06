fn main () {
let result = dangle();
println!("{}",result);
} // result goes oit of scope and is dropped.

fn dangle() -> String {
    let s = String::from("Hello"); // s comes into scope
    s    // onwership is moved out of the function
} // s goes out of the scope 