fn main() {

    // loop {
    //     println!("again!");
    // }


    // Returning Values from Loops:

    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("again!");

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);


    // Conditional Loops with while:

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");


    // Looping Through a Collection with for:
    
    let a = [10, 20, 30, 40, 50];

    // The following is error prone due to indefinite index length:
    
    // let mut index = 0;
    // while index <= 4 {
    //     println!("the value is: {}", a[index]);

    //     index += 1;
    // }

    // A better approach is by using "for":

    for element in a.iter() {
        println!("the value is: {}", element);
    }


    // Here’s what the countdown would look like using a for loop

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

}