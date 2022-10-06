fn main() {
    /* 
    -----Ownership Rules-----
    1. Each value in rust has a variable that's called its owner.
    2. There can only be one owner at a time.
    3. When the owner goes out of scope, the value will be dropped.
    */

    // Basically these two types deal with memory differently - using the stack a size must be known and fixed. On the heap it can have a variable size.
    // In rust memory is automatically returned once the variable that owns it goes out of scope, this is a function called drop.
    let sl: &str = "hello"; // string literal, fixed in size, stored in stack, immutable
    let s: String = String::from("hello"); // String, dynamic in size, stored on the heap, mutable

    // Variables and Data Interact: Move
    let i = 5;  // bind 5 to i
    let ic = i; // copy of i, bind to ic

    let s1 = String::from("hello"); // this will not do the same thing due to being on the heap with a pointer
    let s2 = s1; // this is called a Move. Rust will not make a copy on the heap or a shallow copy pointer. s1 will now move and be s2
    let s3 = s2.clone(); // clone method will make a copy at the expense of memor

    // Ownership and functions
    /*
    fn main() {
        let s: String = String::from("hello");
        take_ownership(some_string: s); // 1. passing s to take_ownership moves s 
        println!("{}", s); // s does not exist
        
    let x: i32 = 5;
        makes_copy(some_integer: x); // integer is copied
        println!("{}", x); // value can still be read here

    let s1: String = gives_ownership();
        println!("s1 = {}", s1);
    
    let s1: String = gives_ownership();
    let s2: String = String::from("hello");
    let s2: String = take_and_gives_back(a_string: s2);
    println!("s1 = {}, s3 = {}", s1, s3);
    }

    fn take_ownership(some_string: String) {
        println!("{}", some_string); // 2. s moved here, after function is run, s is dropped
    }

    fn makes_copy(some_integer: i32) {
        println!("{}", some_integer); // copy can be read here
    }

    fn gives_ownership() => String {
        let give_string: String = String::from("hello");
        give_string  // returns string moves ownership to s1
    }
     
    fn take_and_gives_back(a_string: String) => {
        a_string
    }
    */

    // References

}
