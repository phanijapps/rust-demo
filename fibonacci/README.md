# Fibonacci Sequence in Rust

This Rust program computes the Fibonacci number for a specified value `n`. The Fibonacci sequence is a series of numbers where each number is the sum of the two preceding ones, usually starting with 0 and 1. This application showcases two different methods to calculate the Fibonacci number: iterative and recursive.

## Features

- Calculation of Fibonacci number for a given index `n`.
- Implementation of both iterative and recursive methods to compute Fibonacci numbers.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

You need to have Rust installed on your machine. If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it.

### Compiling and Running

1. Clone the repository to your local machine.
2. Navigate to the directory containing the project.
3. To compile the program, run the following command in your terminal:

```bash
cargo build --release
```

4. To run the program, use:

```bash
cargo run
```

By default, the program calculates the Fibonacci number for `n = 40` using the recursive method. You can modify the source code to change the value of `n` or to switch between the iterative and recursive calculation methods.

## Implementation Details

The program includes two functions for calculating the Fibonacci number:

- `fibonacci_iterative(n: u32) -> u32`: Computes the Fibonacci number iteratively.
- `fibonacci_recursive(n: u32) -> u32`: Computes the Fibonacci number recursively.

The iterative approach is generally more efficient for larger values of `n` due to the recursive method's exponential time complexity and its potential to cause stack overflow errors.

## License

This project is licensed under the MIT License - see the LICENSE.md file for details.

---

You can adjust the README.md content as per your project requirements, such as changing the default value of `n`, explaining more about the Fibonacci sequence, or adding more sections like "Contributing," "Versioning," or "Authors."