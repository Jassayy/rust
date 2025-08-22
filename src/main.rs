//Borrowing and references
//Rihana now says I’d like to be borrowed from time to time. I will still have a single owner, but I can still be borrowed by other variables temporarily. What rules do you think should apply to her?
// She can be borrowed by multiple people that she’s friends with but does no hanky panky
// If she does want to do hanky panky, she can only have 1 borrower that she does it with. She cant simultaneously be with other borrowers (even with no hanky panky)
 
//borrowing
fn main() {
    let my_string: String = String::from("Hello, Rust!");
    borow_variable(&my_string);  // Pass a reference to my_string
    println!("{}", my_string);    // This is valid because ownership was not transferred
}

fn borow_variable(some_string: &String) {
    println!("{}", some_string);  // some_string is borrowed and not moved
}