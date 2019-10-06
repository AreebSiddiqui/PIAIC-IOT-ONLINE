fn main() {
    let a: u8 = 10;  // a has a value 10
    let b=&a; // b points to a 
    let c =&b; // c points to b
    println!("a:{}, b:{}, c:{}",a,b,c);
    println!("The address of a is {:p}",b);
    println!("The address of b is {:p}",c);
}
