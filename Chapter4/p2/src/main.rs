// fn main() {
// let x = 5;
// let y = x;

// }



// fn main () {
//     let a = String::from("hello"); // a is in scope now
//     take_onwership(a); // a is moved into the function take onwership 
//     println!("{}",a); // that's why a is not present here
//     }
// fn take_onwership(a1: String){ // here {
//  println!("{}",a1);
// }

// fn main () {
// let s1 = give_ownership();
// println!("Name: {} ",s1);



// }

// fn give_ownership() -> String {
// let some_string = String::from("Jhon");
// some_string

// }

// fn main () {
//     let a = 10;
//     let b = &a;
//     let f =&b;
//     let c  = &f;
//     println!("a:{}, b:{:p},f:{:p},c:{}",a,b,f,c);
// }

// fn main () {
// for i in 1..11 {
// let result = is_even(i);
// println!("{}. {}",i,result);
// }
// }

// fn is_even(x:i32) -> String {
//     let y = "Even".to_string();
//     let u = "Odd".to_string();
//     // let y =  String::from("Even");
//     // let u =  String::from("Odd");

//     let result = { if x %2 ==0 {
//         y
//     }
//     else {
//         u  
//         }
//         };
//     result
// }



// fn main () {
//      let y = 5 ;
//      let x =  &y;
//      assert_eq!(5,x);
// }



// fn main() {
//     let x = 5;
//     let y = &x;

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// }


// fn main () {
//     let mut s = String::from("Hello");
//     let s1 = &s;
//     let s2 = &s;
//     println!("{}, {}", s1,s2);
// }

fn main () {
    let mut s = String::from("Hello"); //data
    let s1 = &s; //reading of data
    let s2 = &mut s; 
    s2.push_str(", World"); 
    println!("{}",s1);  


}