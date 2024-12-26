fn main() {
    let mut x = 5;
    { // Create a new scope to limit the lifetime of the mutable borrow
        let y = &mut x; 
        *y = 10;
    }
    { // Create a new scope for another mutable borrow
        let z = &mut x;
        *z = 15; 
    }
    println!("x = {}", x);
}
