/*  
    In Rust, variables are declared using the let keyword. 
    All variables are immutable by default, which means once a value is bound to a variable, it cannot be changed. 
    If you want to make a variable mutable, the mut keyword is used. So, 
    if you wanted to declare a mutable variable x and assign it the value 5, you would write let mut x = 5;. 
    Variables can also be patterned. By default in Rust, variables are block-scoped. Rust also supports several types of variable attributes.
 */


fn main() {
    let x = 5; // Immutable variable
    println!("The value of x is: {}", x);

    //x= 10; // This will cause a compile-time error because x is immutable
    //println!("The value of x is: {}", x);
    
    let mut y = 10; // Mutable variable
    println!("The value of y is: {}", y);
    y = 15; // Changing the value of y
    println!("The new value of y is: {}", y);
    
}
