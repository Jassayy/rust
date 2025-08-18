fn main() {
    let is_even = true;

    if is_even {
        println!("the number is even");
    } else if !is_even {
        println!("the number is odd");
    }

    // loops
    for i in 0..11 {
        println!("{}", i);
    }

    let sentence: String = String::from("my name is jas");
    let first_word = get_first_word(&sentence);//passing only ref on sentence as in rust if we dont use & we transfer ownership into the function and hence we wont be able to use "sentence afterwards"

    println!("{}", first_word); 
}
//this is how we can iterate over strings or even arrays or maps etc
// fn function_name(args) -> return type { }
fn get_first_word(sentence: &str) -> String {
    let mut ans: String = String::from("");
    for char in sentence.chars() {
        if char == ' ' {
            break;
        }
        ans.push(char);
    }
    ans
}
