fn main() {
    let my_string = String::from("hello");
    let my_string3 =  takes_ownership(my_string);
    println!("{}", my_string3); // This line would cause a compile error because ownership has been moved.
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string); // `some_string` now owns the data.
    //can rihana go back to her previous boyfriend? yes just return
    return some_string;
}


//in this example rihana moves from boyfriend 1 -> boyfriend 2 and as he is about to die
//rihana moves to boyfriend 3
//what did we learn? rihana is a whore