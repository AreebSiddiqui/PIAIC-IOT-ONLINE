fn main() {
  let mut s = String::from("Hello");
  change(&s);
}

fn change(x: &String) {
x.push_str("World");
println!("{}",x);
}
