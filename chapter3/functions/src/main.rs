// fn main() {
//   println!("BAKER SHARING RECIPE WITH FIRST PERSON");
//     paper();    


//   println!("BAKER SHARING RECIPE WITH SECOND PERSON");
//   paper();



//   println!("BAKER SHARING RECIPE WITH THRID PERSON");
//    paper();
  


// }

// fn paper() {
//      println!("1. Add Milk");
//      println!("2. Add Butter");
//      println!("3. Add eggs");
//      println!("4. Add Sugar");
//      println!("5. Stir it");
//      println!("6. Heat on gentle flame"); 
// }





fn main () {
let (value,value_1) = square(2,9.1); //Arguments
println!("{}, {}",value,value_1);
}


fn square(x: u32,y:f64) -> (u32,f64) {//parameters 
let result = x* x; //statement
let result_1 = y*y; //statement

(result,result_1)


}


