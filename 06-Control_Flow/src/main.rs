fn main() {
    // if Expressions:
    
    let number = 4;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 0;
    if number != 0 {
        println!("number was something other than zero");
    } else {
        println!("It is now zero");
    }


    // Handling Multiple Conditions with else if:

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    // Using if in a let Statement:

    let condition = false;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

}
