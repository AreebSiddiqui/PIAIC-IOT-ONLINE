// // fn main () {
// //     let x=5;
// //     let y=x;
// //     println!("{},{}",x,y);

// //     let x = String::from("Hello world");
// //     let y =x;
// //     println!("{},{}",x,y);


// // }
// // let s1 = String::from("hello");
// // let s2 = s1;


// fn main () {
//     let x = 5 ;  //integer -> Stack
//     let y = x ;
//     println!("{} , {}",x,y ); //stack 


//     let s1 = String::from("Hello"); // String -> Heap
//     let s2  = s1;
    

// }  // s2 drop













fn main () {
    let s1 = String::from("Hello"); //s comes into scope
    take_ownership(s1); //s1 value moved in to the function and no longer valid here.
    println!("{}",s1); 
    let x =5;  //x comes into scope
    makes_copy(x); // x would move into the function but i32 is copy so its ok to use x afterwards
    println!("x");
} 

fn take_ownership(some_string: String) {
println!("{}",some_string);
} // Here some string goes out of the scope and some_string is dropped backing up the memory

fn makes_copy(some_integer: i32) {
println!("{}",x);
}