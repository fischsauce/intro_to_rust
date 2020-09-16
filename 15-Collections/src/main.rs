
// Storing Lists of Values with Vectors

//  Vec<T>, also known as a vector allow you to store more than one 
// value in a single data structure that puts all the values next to each 
// other in memory.


fn main() {
    
    let v: Vec<i32> = Vec::new();

    
    // or, using the vec! macro will create a new vector that holds the values you give it:
    let v2 = vec![1, 2, 3];  // type is inferred


    // To add elements to a vector, we can use the push method:

    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    aux();
    

    // a vector is freed when it goes out of scope:

}   // <- v goes out of scope and is freed here


fn aux() {

    // There are two ways to reference a value stored in a vector.

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(4) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }


    // let’s see what a program will do if it has a vector that holds five 
    // elements and then tries to access an element at index 100:

    let v = vec![1, 2, 3, 4, 5];

    // the first [] method will cause the program to PANIC because it references 
    // a nonexistent element. This method is best used when you want your program to 
    // CRASH if there’s an attempt to access an element past the end of the vector.
    
    // let does_not_exist = &v[100]; 
                                    // thread 'main' panicked at 'index out of bounds: 
                                    // the len is 5 but the index is 100'


    // When the get method is passed an index that is outside the vector, it returns 
    // None WITHOUT PANICKING. You would use this method if accessing an element beyond 
    // the range of the vector happens occasionally under normal circumstances.
    
    let does_not_exist = v.get(100);

    tertiary();

}


// Iterating over the Values in a Vector

fn tertiary() {

    // we can iterate through all of the elements rather than use 
    // indices to access one at a time:

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }


    // We can also iterate over mutable references to each element in 
    // a mutable vector in order to make changes to all the elements:

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // To change the value that the mutable reference refers to, we have to 
    // use the dereference operator (*) to get to the value in i before we can 
    // use the += operator.

}

// Using an Enum to Store Multiple Types

fn quattro() {

    // the variants of an enum are defined under the same enum type, so when we need 
    // to store elements of a different type in a vector, we can define and use an enum!

    // We can define an enum whose variants will hold the different value types, and then 
    // all the enum variants will be considered the same type: that of the enum:
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // Then we can create a vector that holds that enum and so, ultimately, 
    // holds different types:
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];


    // if you don’t know the exhaustive set of types the program will get at runtime 
    // to store in a vector, the enum technique won’t work. Instead, 
    // you can use a trait object
}