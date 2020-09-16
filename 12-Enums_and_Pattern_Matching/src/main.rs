
// Rust’s enums are most similar to algebraic data types in functional languages, 
// such as F#, OCaml, and Haskell.

// Defining an Enum:

// * enum values can only be one of its variants.

fn main() {
    
    enum IpAddrKind {
        V4,
        V6,
    }

    // IpAddrKind is now a custom data type that we can use elsewhere in our code.


// Enum Values:

    // We can create instances of each of the two variants of IpAddrKind like this:

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // We can then, for instance, define a function that takes any IpAddrKind:
    fn route(ip_kind: IpAddrKind) {}

    // And we can call this function with either variant:
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);


    // Here, we’ve defined a struct IpAddr that has two fields:

    struct IpAddr {
        // a kind field that is of type IpAddrKind:
        kind: IpAddrKind,

        // and an address field of type String:
        address: String,
    }


    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // We’ve used a struct to bundle the kind and address values together, 
    // so now the variant is associated with the value.


    // We can represent the same concept in a more concise way using just an enum, 
    // rather than an enum inside a struct, by putting data directly into each enum variant:

    enum IpAddr2 {
        V4(String),
        V6(String),
    }

    let home = IpAddr2::V4(String::from("127.0.0.1"));

    let loopback = IpAddr2::V6(String::from("::1"));

    // We attach data to each variant of the enum directly, so there is no need for an extra struct.


    // each variant can have different types and amounts of associated data:

    enum IpAddr3 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr3::V4(127, 0, 0, 1);

    let loopback = IpAddr3::V6(String::from("::1"));


    // Let’s look at how the standard library defines IpAddr: 
    
    // it has the exact enum and variants that we’ve defined and used, but it embeds
    // the address data inside the variants in the form of two different structs, 
    // which are defined differently for each variant:

    struct Ipv4Addr {
        // --snip--
    }
    
    struct Ipv6Addr {
        // --snip--
    }
    
    enum IpAddr4 {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }


    // Let’s look at another example of an enum. 
    
    // this one has a wide variety of types embedded in its variants:

    #[derive(Debug)]

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }


    // The following structs could hold the same data that the preceding enum variants hold:

    // struct QuitMessage; // unit struct
    // struct MoveMessage {
    //     x: i32,
    //     y: i32,
    // }
    // struct WriteMessage(String); // tuple struct
    // struct ChangeColorMessage(i32, i32, i32); // tuple struct


    // There is one more similarity between enums and structs: just as we’re able to
    // define methods on structs using impl, we’re also able to define methods on enums.
    // Here’s a method named call that we could define on our Message enum:

    

    impl Message {
        fn call(&self) {
            println!("{:?}", &self)
            
        }
    }

    let m = Message::Write(String::from("hello"));

    m.call();
}



// Rust does not have nulls, but it does have an enum that can encode the concept 
// of a value being present or absent. This enum is Option<T>, and it is defined 
// by the standard library as follows:

enum Option<T> {
    Some(T),
    None,
}

// you can use Some and None directly without the Option:: prefix. The Option<T> enum
// is still just a regular enum, and Some(T) and None are still variants of type Option<T>.



// The <T> syntax is a feature of Rust. <T> means the Some variant of the Option enum
//  can hold one piece of data of any type. 

// Here are some examples of using Option values to hold number types and string types:


// let some_number = Some(5);   

// let some_string = Some("a string");

// let absent_number: Option<i32> = None; // If we use None rather than Some, we need to
                                          // tell Rust what type of Option<T> we have, 
                                          // because the compiler can’t infer the type.