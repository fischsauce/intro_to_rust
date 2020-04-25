fn main() {             // s is not valid here, it’s not yet declared
    let s = "hello";    // s is valid from this point forward
    
                        // do stuff with s
                        
    strings();
    mover();
}                       // this scope is now over, and s is no 
                        // longer valid


fn strings() {

    // This kind of string can be mutated:
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
}


// Ways Variables and Data Interact: Move
fn mover() {
    let x = 5;
    let y = x;

    // Now let’s look at the String version:
    let s1 = String::from("hello");
    let s2 = s1;

    // When we assign s1 to s2, the String data is copied, 
    // meaning we copy the pointer, the length, and the 
    // capacity that are on the stack. We do not copy the 
    // data on the heap that the pointer refers to.

    // Rust considers s1 to no longer be valid and, therefore, 
    // Rust doesn’t need to free anything when s1 goes out of scope. 
    
    // Check out what happens when you try to use s1 
    // after s2 is created; it won’t work:

    // println!("{}, world!", s1);
                            // ^^ value borrowed here after move

    // In this example, we would say that s1 was moved into s2
}


// Ways Variables and Data Interact: Clone
fn cloner() {
    // If we do want to deeply copy the heap data of the String, not
    // just the stack data, we can use a common method called clone.

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // When you see a call to clone, you know that some arbitrary code 
    // is being executed and that code may be expensive.
}


// Stack-Only Data: Copy
fn copier() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // But this code seems to contradict what we just learned: we don’t 
    // have a call to clone, but x is still valid and wasn’t moved into y.

    // The reason is that types such as integers that have a known size 
    // at compile time are stored entirely on the stack, so copies of 
    // the actual values are quick to make. 

    // Rust has a special annotation called the Copy trait that we can 
    // place on types like integers that are stored on the stack.

    // If a type has the Copy trait, an older variable is still usable 
    // after assignment.

    // Here are some of the types that are Copy:

        // All the integer types, such as u32.
        // The Boolean type, bool, with values true and false.
        // All the floating point types, such as f64.
        // The character type, char.
        // Tuples, if they only contain types that are also Copy. 
            // For example, (i32, i32) is Copy, but (i32, String) is not.
}


// Ownership and Functions
fn foo() {
    // Passing a variable to a function will move or copy, 
    // just as assignment does.

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into 
                                    // the function...

                                    // ... and so is no longer
                                    //  valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay 
                                    // to still use x afterward

}   // Here, x goes out of scope, then s. But because s's value 
    // was moved, nothing special happens.


fn takes_ownership(some_string: String) { // some_string comes 
                                          // into scope
    println!("{}", some_string);

}   // Here, some_string goes out of scope and `drop` is called. 
    // The backing memory is freed.


fn makes_copy(some_integer: i32) { // some_integer comes into scope
    
    println!("{}", some_integer);

}   // Here, some_integer goes out of scope. Nothing special happens.


// Return Values and Scope
fn bar() {
    // Returning values can also transfer ownership.

    let s1 = gives_ownership();         // gives_ownership moves 
                                        // its return value into s1

    let s2 = String::from("hello");     // s2 comes into scope


    let s3 = takes_and_gives_back(s2);  // s2 is moved into 
                                        // takes_and_gives_back, 
                                        // which also moves its 
                                        // return value into s3

}   // Here, s3 goes out of scope and is dropped.
    // s2 goes out of scope but was moved, so nothing happens. 
    // s1 goes out of scope and is dropped.


fn gives_ownership() -> String {    // gives_ownership will move 
                                    // its return value into the 
                                    // function that calls it

    let some_string = String::from("hello"); // some_string comes 
                                             // into scope

    some_string                     // some_string is returned and 
                                    // moves out to the calling function
}

// takes_and_gives_back will take a String and return one:
fn takes_and_gives_back(a_string: String) -> String { // a_string comes 
                                                      // into scope

a_string  // a_string is returned and moves out to the calling function

}

// When a variable that includes data on the heap goes out of scope, 
// the value will be cleaned up by drop unless the data has been moved 
// to be owned by another variable.


// What if we want to let a function use a value but not take ownership?


// It’s possible to return multiple values using a tuple:
fn returner() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}