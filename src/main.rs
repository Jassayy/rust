// //print hellow world
// fn main() {
//     println!("Hello, world!");
// }

//simple variables in rust
fn main(){
    let mut x: i32 = 1; //32 bit number .. 'i' means signed integer ...'u' for unsigned f for float etc etc
   

    for i in 0..100{
            x = x + 100; //value of x is changing so add mut keyword while defining the var as it is mutable
    }
     println!("x: {}" , x);
}