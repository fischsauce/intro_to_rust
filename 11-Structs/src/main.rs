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


// Refactoring with Structs: Adding More Meaning:

#[derive(Debug)]

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



