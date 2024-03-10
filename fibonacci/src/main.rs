use clap::{App, Arg};

fn main() {
     let matches = App::new("fibonacci")
         .version("1.0")
         .arg(
             Arg::with_name("number")
                 .short('n')
                 .long("number")
                 .value_name("NUMBER")
                 .help("Sets number to find fibonacci of")
                 .takes_value(true)
                 .default_value("20")
                 .required(false),
         )
         .arg(
             Arg::with_name("algo")
                 .short('a')
                 .long("algorithm")
                 .value_name("ALGO")
                 .help("Sets the action mode (I for iterative, R for recursive)")
                 .possible_values(&["I", "R"])
                 .takes_value(true)
                 .default_value("I")
                 .required(false),
         )
         .get_matches();
 
     let number: i32 = matches
         .value_of("number")
         .unwrap() // It's safe to use unwrap because we made it required
         .parse()
         .expect("Number needs to be an integer");
 
     let action = matches.value_of("algo").unwrap(); // Also safe to unwrap
 
     println!("Number: {}", number);
     println!("Algo: {}", action);

    let mut fib: i32 = 0;
    if action == "I" {
        fib = fibonacci_iterative(number);
    }else if action == "R" {
        fib = fibonacci_recursive(number);
    }
    
    println!("Fibonacci of {} is {}", number, fib);
}

fn fibonacci_iterative(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }
    a
}

//Recursive Fibonacci Number Series
// 
fn fibonacci_recursive(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}
