// Concise Control Flow with if let


fn main() {
    
    // Consider the following program that matches on an Option<u8> value but only wants to execute code if the value is 3:

    let some_u8_value = Some(0u8);

    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }


    // Instead, we could write this in a shorter way using if let. The following code behaves the same:

    if let Some(3) = some_u8_value {
        println!("three");
    }

    // you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.


    // We can include an else with an if let. The block of code that goes with the else is the same as the block of code that would go with the _ case in the match expression that is equivalent to the if let and else:

    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // Or we could use an if let and else expression like this:

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

}
