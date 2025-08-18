fn main(){
    // let _ax = "kajsdib"; //can change space at runtime
    //therefore needs more memory from the ram at runtime
    //therefore we need to ask ram for more memory
    let greeting: String = String::from("hello world");

    println!("{}" ,greeting);
     let char: Option<char> = greeting.chars().nth(1); //type pf this optionally a char

     //error: `Option<char>` doesn't implement `std::fmt::Display`
     // the trait `std::fmt::Display` is not implemented for `Option<char>`
    print!("{}" , char.unwrap()); //to get nth char of string
    //therfore we cant do this.. we hve to "match"
    //unwrap here works but dont use it 
}