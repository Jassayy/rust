//Borrowing and references
//Rihana now says I’d like to be borrowed from time to time. I will still have a single owner, but I can still be borrowed by other variables temporarily. What rules do you think should apply to her?
// She can be borrowed by multiple people that she’s friends with but does no hanky panky
// If she does want to do hanky panky, she can only have 1 borrower that she does it with. She cant simultaneously be with other borrowers (even with no hanky panky)
 
//reference
fn main() {
    let s1 = String::from("Hello");
    let s2 = &s1; //s1 is borrowed here

    println!("{}", s2);
    println!("{}", s1);    // This is valid, The first pointer wasn't invalidated
}