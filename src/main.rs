//jargon 1
//mutability
//Immutable variables represent variables whose value cant be changed once assigned

//by default variables are immutable
//can use "mut" keyword for making it mutable
//knowing a data wont change will make the compiler to optimize better

fn main() {
    let mut x: i32 = 1;
    x = 2; //no error as we used mut
    println!("{}", x);
}