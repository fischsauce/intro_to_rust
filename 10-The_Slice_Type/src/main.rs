// The Slice Type

// Slices let you reference a contiguous sequence of elements in a 
// collection rather than the whole collection.
fn main() {
    let mut sentence = String::from("I've have a lot of apples");
    
    let word = first_word(&sentence);   // word will get the value 4
                 
    sentence.clear();                   // this empties the String, 
                                        // making it equal to ""

                            // word still has the value 5 here, but 
                            // there's no more string that we could 
                            // meaningfully use the value 5 with. word 
                            // is now totally invalid!

    println!("{}", word);  
}

// We now have a way to find out the index of the end of the first word 
// in the string, but there’s a problem. We’re returning a usize on its 
// own, but it’s only a meaningful number in the context of the &String

// We could use that value 4 with the variable 'sentence' to try to 
// extract the first word out, but this would be a bug because the 
// contents of 'sentence' have changed since we saved 4 in 'word'.

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// Luckily, Rust has a solution to this problem: string slices.


// String Slices

// We can create slices using a range within brackets by specifying 
// [starting_index..ending_index], where starting_index is the first 
// position in the slice and ending_index is one more than the last 
// position in the slice:
fn slices() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}

// With Rust’s .. range syntax, if you want to start at the first index 
// (zero), you can drop the value before the two periods. In other 
// words, these are equal:

    // let s = String::from("hello");

    // let len = s.len();

    // let slice = &s[3..len];
    // let slice = &s[3..];


    // You can also drop both values to take a slice of the entire 
    // string. So these are equal:

        // let s = String::from("hello");

        // let len = s.len();

        // let slice = &s[0..len];
        // let slice = &s[..];


// With all this information in mind, let’s rewrite first_word to 
// return a slice. The type that signifies “string slice” is written as 
// &str:
fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// We now have a straightforward API that’s much harder to mess up, 
// because the compiler will ensure the references into the String 
// remain valid. 


// Other Slices

// there’s a more general slice type, too. Consider this array:

    // let a = [1, 2, 3, 4, 5];

    // let slice = &a[1..3];

