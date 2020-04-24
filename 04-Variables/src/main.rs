fn main() {

    // The following won't compile, because variables are immutable by default
    
    // let x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The valus of x is: {}", x);
    
    // Output:

    //     x = 6;
    //   | ^^^^^ cannot assign twice to immutable variable


    // Here is the correct approach:
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The valus of x is: {}", x);


    // Constants are always immutable, and must be declared along with a type annotation:
    const MAX_POINTS: u32 = 100_000;


    // Shadowing can be used to inherit the previous value: 
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    // ...it can also be used to change the variable type:
    let spaces = "   ";         // String
    let spaces = spaces.len();  // Number

    // ...however if we make the variable mutable, we will get a compiler error:
    
    // let mut spaces = "   ";
    // spaces = spaces.len();
    // ^^^^^^^^^^^^ expected `&str`, found `usize`

}