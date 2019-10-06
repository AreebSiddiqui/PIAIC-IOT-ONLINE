fn main () {
let s = String::from("Pakistan"); 
let result = lenght(&s);
println!("The lenght of the word {} is {}",s,result);
}

fn lenght(x:&String) -> usize {
x.len()
}