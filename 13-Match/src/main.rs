// The match Control Flow Operator

// Allows you to compare a value against a series of patterns and then execute code 
// based on which pattern matches.


// We can write a function that can take an unknown United States coin and, in a 
// similar way as the counting machine, determine which coin it is and return its 
// value in cents:

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {        // the => operator separates the pattern and the code to run
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {

    let coin1 = Coin::Dime;
    let coin2 = Coin::Penny;
    
    println!("{}", value_in_cents(coin1));
    println!("{}", value_in_cents(coin2));

    println!("{}", value_in_centss(Coins::Quarter(UsState::Alaska)));

}


// Patterns that Bind to Values

// Another useful feature of match arms is that they can bind to the parts of the values that match the pattern. This is how we can extract values out of enum variants.

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_centss(coin: Coins) -> u8 {
    match coin {
        Coins::Penny => 1,
        Coins::Nickel => 5,
        Coins::Dime => 10,
        Coins::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}


// Matching with Option<T>

// Let’s say we want to write a function that takes an Option<i32> and, if there’s a value
// inside, adds 1 to that value. If there isn’t a value inside, the function should return
// the None value and not attempt to perform any operations:

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main2() {

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // Combining match and enums is useful in many situations. You’ll see this pattern a 
    // lot in Rust code: match against an enum, bind a variable to the data inside, and 
    // then execute code based on it.


    // Matches Are Exhaustive

    // Consider this version of our plus_one function that has a bug and won’t compile:

    //     fn plus_one(x: Option<i32>) -> Option<i32> {
    //        match x {
    //              ^ pattern `None` not covered
    //           Some(i) => Some(i + 1),
    //        }
    //     }

    // We didn’t handle the None case, so this code will cause a bug.



    // The _ Placeholder

    let some_u8_value = 0u8;

    // Rust also has a pattern we can use when we don’t want to list all possible values.
    //  For example, a u8 can have valid values of 0 through 255. If we only care about the 
    //  values 1, 3, 5, and 7, we don’t want to have to list out 0, 2, 4, 6, 8, 9 all the way 
    //  up to 255. Fortunately, we don’t have to: we can use the special pattern _ instead:

    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    // The _ pattern will match any value. By putting it after our other arms, the _ will
    // match all the possible cases that aren’t specified before it. The () is just the unit
    // value, so nothing will happen in the _ case. 

}
