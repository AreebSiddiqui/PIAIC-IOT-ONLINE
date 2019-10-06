fn main() {                        
   let student: (u32,char,f64) = (25,'A',80.5);
println!("{}",student.0); //25
println!("{}",student.1); //'A'
println!("{}",student.2); //80.5

println!("Destructure");
let (x,y,z) = student;
println!("{}",x);
println!("{}",y);
println!("{}",z);


}
