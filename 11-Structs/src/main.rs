
// A basic struct:

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Returning a struct from a function:

fn build_user(email: String, username: String) -> User {

    // Because the parameter names and the struct field names are exactly 
    // the same in, we can use the field init shorthand syntax

    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }

    // Or, to conSTRUCT in a static fashion:

    // let user1 = User {
    //     email: String::from("someone@example.com"),
    //     username: String::from("someusername123"),
    //     active: true,
    //     sign_in_count: 1,
    // };


    // Using struct update syntax, we can achieve the same effect with less code:

    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     username: String::from("anotherusername567"),
    //     ..user1        // remaining values are set from the previous instance of User
    // };

}


// You can also define structs that look similar to tuples, called tuple structs. 
// Tuple structs have the added meaning the struct name provides but don’t have names
// associated with their fields; rather, they just have the types of the fields.


struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// let black = Color(0, 0, 0);
// let origin = Point(0, 0, 0);


// Each struct you define is its own type, even though the fields 
// within the struct have the same types.



// It’s possible for structs to store references to data owned by something else, 
// but to do so requires the use of lifetimes. Lifetimes ensure that the data 
// referenced by a struct is valid for as long as the struct is. See chapter 10.





fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    main2();
    main3();
    main4();
    main5();
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}


// Refactoring with Tuples:

fn main2() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}


// Refactoring with Structs::



#[derive(Debug)]    // structs don’t have a provided implementation of Display, 
                    // becauase ambiguity of types. ergo this is necessary.

struct Rectangle {
    width: u32,
    height: u32,
}

fn main3() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect1)
    );

    printRect(&rect1);
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


// Adding Useful Functionality with Derived Traits:

fn printRect(rectangle: &Rectangle) {
    println!("rect1 is {:?}", rectangle);
    
    println!("rect1 is {:#?}", rectangle);

}


// Method Syntax - Defining Methods:

// Let’s change the area function that 
// has a Rectangle instance as a 
// parameter and instead make an area 
// method defined on the Rectangle struct

#[derive(Debug)]

struct Rectanglo {
    width: u32,
    height: u32,
}

impl Rectanglo {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn border(&self) -> u32 {
        (self.width + self.height) * 2
    }

    // Methods with More Parameters:
    fn can_hold(&self, other: &Rectanglo) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated Functions:
    fn square(size: u32) -> Rectanglo {
        Rectanglo {
            width: size,
            height: size,
        }
    }
    // To call this associated function, we use the :: syntax with the struct name; let sq = Rectangle::square(3); is an example.
}

fn main4() {
    let rect1 = Rectanglo {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "The border length of the  rectangle is {} pixels", rect1.border()
    );
}


// Methods with More Parameters:

fn main5() {
    let rect1 = Rectanglo {
        width: 30,
        height: 50,
    };
    let rect2 = Rectanglo {
        width: 10,
        height: 40,
    };
    let rect3 = Rectanglo {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// A note about method operators; 

// Rust has a feature called automatic referencing and dereferencing. when you call a method 
// with object.something(), Rust automatically adds in &, &mut, or * so object matches the 
// signature of the method. 

// In other words, the following are the same:

// p1.distance(&p2);
// (&p1).distance(&p2);

