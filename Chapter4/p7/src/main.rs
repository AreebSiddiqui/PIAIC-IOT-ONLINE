fn main() {
let mut s = String::from("Hello");
let b = &s;
let c = &s;
let d = &s;

println!("{},{}, {}", b,c,d);

let a = &mut s;
a.push_str("World");
println!("{}",a);




}