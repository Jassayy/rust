//practice 
fn main(){

    let n: i32 = 5;

    let ans: i32 = fib(n);

    print!("{}th number in fib series is: {}" , n , ans );
}

fn fib(n : i32) -> i32 {

    
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    return fib(n-1) + fib(n-2);
}

// 0 1 1 2 3 5