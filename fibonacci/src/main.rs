fn main() {
    //let args: Vec<String> = env::args().collect();

    let n = 100;
    let fib = fibonacci_recursive(n);
    println!("Fibonacci of {} is {}",n,fib);
}

fn fibonacci_iterative(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }
    a
}

fn fibonacci_recursive(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}
