fn main() {
    another_function(5, 6);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let z = five();
    println!("The value of z is: {}", z);

    let q = plus_one(5);
    println!("The value of q is: {}", q);
}

fn another_function(x: i32, y:u32) {
    println!("The value of x is: {}, the value of y is: {}", x, y);
}

fn five() -> i32 {
    555
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

