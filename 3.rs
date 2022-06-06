fn main() {
    let mut arr:[i32;10] = [0,1,2,3,4,5,6,7,8,9];
    println!("Array : {:?}",arr);

    println!("Array elements in reverse order: ");
    for i in (0..arr.len()).rev(){
        println!("{}",arr[i]);
    }

    // reversing array
    arr.reverse();
    println!("Reversed array : {:?}",arr);
    
}