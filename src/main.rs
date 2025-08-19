//jargon 2 
//stack vs heap
//stack is used to store fixed size data structures like arrays which have a particular size only in memory
//heap will be used to store dynamic structures like vectors and strings etc which dont have a fixed size and will change when required
//in heap we have to ask the os to allocate space...if we need more we can ask more
//much slower than stack

// Stack: Fast allocation and deallocation. Rust uses the stack for most primitive data types and for data where the size is known at compile time (eg: numbers).
// Heap: Used for data that can grow at runtime, such as vectors or strings.

//while storing strings -> it is still stored in the stack but not the actual values
//mem address of the char of string are stored in stack and the string itself is stored in heap
//pointers , len , capacity etc will be stored in the stack


fn main() {
    stack_fn();   // Call the function that uses stack memory
    heap_fn();    // Call the function that uses heap memory
    update_string();  // Call the function that changes size of variable at runtime
}

fn stack_fn() {
    // Declare a few integers on the stack
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("Stack function: The sum of {} and {} is {}", a, b, c);
}

fn heap_fn() {
    // Create a string, which is allocated on the heap
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{} {}", s1, s2);
    println!("Heap function: Combined string is '{}'", combined);
}

fn update_string() {
    // Start with a base string on the heap
    let mut s = String::from("Initial string");
    println!("Before update: {}", s);

    //lets print the capacity length and pointer
    println!(" capacity : {} , length : {} , pointer : {:p}" , s.capacity() , s.len() , s.as_ptr());

    // Append some text to the string
    s.push_str(" and some additional text");
    println!(" capacity : {} , length : {} , pointer : {:p}" , s.capacity() , s.len() , s.as_ptr());

    //capacity : 14 , length : 14 , pointer : 0x1f2f2eb92e0
    //  capacity : 39 , length : 39 , pointer : 0x1f2f2eb3820
    //we can see the memory address has changed and the length and capacity as well
    //pointer will not change if theres availble contiguous memory...if t all not contiguous memory is not available
    //whole string will get stored in another memory address and therfore change
    //capacity can be more than length

    println!("After update: {}", s);
}
