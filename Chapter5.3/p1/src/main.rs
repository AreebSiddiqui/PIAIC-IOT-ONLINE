struct Rectangle {
    heigth:u32,
    width:u32,
}

impl Rectangle {
    fn area (&self) -> u32 {  //now compiler is here
        self.width*self.heigth
    }
    
}

fn main() {
    let rec_1 = Rectangle {heigth:100, width:50}; //heigth , width
    let rec_2 = Rectangle {heigth:90, width:10}; //10  //90
    let result = rec_1.area(); //call // program is run till here
    let result_1 = rec_2.area();
    println!("The area of Rectangle is: {}",result);
     println!("The area of 2nd Rectangle is: {}",result_1);
}