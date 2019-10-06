struct Rectangle {
    height :u32,
    width:u32,
}
impl Rectangle {
    fn can_hold (&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
        //100      >    90         &&   50       >   40
        // true                    &&    true
        
    }
}


fn main () {

let rec_1 = Rectangle {height:100, width:50};
let rec_2 = Rectangle {height:90, width:40};
let rec_3 = Rectangle {height:70, width:30};

let result = rec_1.can_hold(&rec_2); // can rec1 hold rec2 ?
println!("Rec_1 can hold Rec_2 : {}",result);
println!("Rec_1 can hold Rec_3: {}",rec_1.can_hold(&rec_3));
}