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

// The issue with the tuple code above is that we have to return the 
// String to the calling function so we can still use the String after 
// the call to calculate_length, because the String was moved into 
// calculate_length.


// References and Borrowing

// Here is how you would define and use a calculate_length function 
// that has a reference to an object as a parameter instead of taking 
// ownership of the value:
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length2(&s1);   // These ampersands are 
                                        // references, and they allow 
                                        // you to refer to some value 
                                        // without taking ownership of it.


    println!("The length of '{}' is {}.", s1, len);
}


fn calculate_length2(s: &String) -> usize { // s is a reference to 
                                            // a String s.len()
    s.len()

}   // Here, s goes out of scope. But because it does not have 
    // ownership of what it refers to, nothing happens.


    // We call having references as function parameters borrowing.


// So what happens if we try to modify something we’re borrowing?
// fn borrow() {
//     let s = String::from("hello");
//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
    // ^^^^^^^^^^^ `some_string` is a `&` reference, so the data it 
    //              refers to cannot be borrowed as mutable
// }

// We’re not allowed to modify something we have a reference to.


// ** The opposite of referencing by using & is dereferencing, which is 
// ** accomplished with the dereference operator, *


// Mutable References

// We can fix the error in the above code:
fn borrow2() {
    let mut s = String::from("hello");
    change2(&mut s);
}

fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}

// But mutable references have one big restriction: you can have only 
// one mutable reference to a particular piece of data in a particular 
// scope.

// eg, this code will fail:
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s;

//     println!("{}, {}", r1, r2);


// we can use curly brackets to create a new scope, allowing for 
// multiple mutable references, just not simultaneous ones:
fn works() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;

    }   // r1 goes out of scope here, so we can make a new reference 
        // with no problems.

    let r2 = &mut s;
}

// A similar rule exists for combining mutable and immutable 
// references. This code results in an error:

//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
            //  ^^ immutable borrow occurs here

//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG PROBLEM
            //  ^^^^^^ mutable borrow occurs here

//     println!("{}, {}, and {}", r1, r2, r3);
                            //    ^^ immutable borrow later used here

                                                 
// This code will compile because the last usage of the immutable 
// references occurs before the mutable reference is introduced:
fn work2() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

// The scopes of the immutable references r1 and r2 end after the 
// println! where they are last used, which is before the mutable 
// reference r3 is created. These scopes don’t overlap, so this code is 
// allowed.


// Dangling References

// Let’s try to create a dangling reference, which Rust will prevent 
// with a compile-time error:

        // fn main() {
        //     let reference_to_nothing = dangle();
        // }

        // fn dangle() -> &String {         // dangle returns a     
                                            // reference to a String

        //     let s = String::from("hello");   // s is a new String

        //     &s                           // we return a reference to 
                                            // the String, s

        // }                                // Here, s goes out of 
                                            // scope, and is dropped. 
                                            // Its memory goes away.
                                            // Danger!

    // Because s is created inside dangle, when the code of dangle 
    // is finished, s will be deallocated.

    // The solution here is to return the String directly:

    fn no_dangle() -> String {
        let s = String::from("hello");
    
        s
    }