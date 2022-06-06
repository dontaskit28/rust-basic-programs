struct Shape{
    length:i32,
    width:i32
}
impl Shape{
    fn area(&self) -> i32{
        self.length * self.width
    }
    fn perimeter(&self) -> i32 {
        2 * (self.length+self.width)
    }
}
fn main(){
    let shape = Shape{length:5,width:2};
    println!("Area: {}",shape.area());
    println!("Perimeter: {}",shape.perimeter());
}