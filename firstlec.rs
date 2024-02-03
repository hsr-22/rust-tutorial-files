// let x: u32 = 10; unsigned 32
// let y: f64 = 3.14; float 64
// let x = 5; rust will infer the type of x

// i is a 32-bit signed integer

// Function syntax : 
// fn function_name(parameter_name: type) -> return_type {
//     // function body
// }

// control flow syntax: as expected

// match syntax: 
// match value {
//     pattern => expression,
//     pattern => expression,
//     pattern => expression,
// }

// ownership rules, chapter 4 in rust textbook

// string does not implement copy trait, so it is moved when assigned to another variable.
// the pointer to the string is moved to the new variable, and the old variable is invalidated.
// to clone a string, use the clone method. but this is an expensive operation.
// can use the & operator to create a reference to the string, which does not take ownership of the string.
// creating a reference is called borrowing, and the reference is valid until the end of the scope in which it was created.

// mutable references are also allowed, but only one mutable reference is allowed at a time.

// slices are references to a part of a string, and are immutable.

// rust doesn't have a garbage collector, so it uses the concept of ownership to manage memory.

// rust doesn't have classes, but it has structs, which are similar to classes.