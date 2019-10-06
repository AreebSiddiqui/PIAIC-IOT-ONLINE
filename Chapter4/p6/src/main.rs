fn main() {
   let mut s = String::from("Hello");
   
   {
       let a = &mut s;
       a.push_str("World");
       println!("{}",a);
   }
   {
        let b = &mut s;
        println!("{}",b);
   }
 
}
