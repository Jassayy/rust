//functions
fn main(){
    let a = 4;
    let b = 18;

    let ans = do_sum(a, b);

    print!("{}" , ans);
}


//this function return i32 integer
fn do_sum(a: i32, b: i32) -> i32 {
	return a + b;
}