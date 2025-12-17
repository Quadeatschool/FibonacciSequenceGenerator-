# Overview

As an aspiring software engineer, I am interested in expanding my proficiency in systems-level programming. My goal with this project was to figure out a new programming language, Rust in this case. By building this generator, I am deepening my understanding of programming in general and testing out basic Rust capabilities.

The software is a Fibonacci Sequence Generator built in Rust. It captures user input from the terminal, validates that the input is a positive integer, and then generates the Fibonacci sequence up to that number. The core logic is decoupled into a specialized function that leverages mutable references to populate a dynamic data structure. To demonstrate advanced memory handling, the program also provides a "snapshot" of the data using memory slicing.

My purpose for writing this software was to apply the core pillars of Rust: Ownership, Borrowing, and Pattern Matching. I wanted to see how the language handles common programming tasks—like looping and variable mutation—while strictly enforcing safety rules at compile time.

{Provide a link to your YouTube demonstration. It should be a 4-5 minute demo of the software running and a walkthrough of the code. Focus should be on sharing what you learned about the language syntax.}

[Software Demo Video](http://youtube.link.goes.here)

# Development Environment

Visual Studio Code: My primary IDE, configured with the rust-analyzer extension for real-time type checking and linting.

Cargo: Rust’s package manager, used for initializing the project, managing dependencies, and compiling the binary.

Rustc: The Rust compiler, ensuring all ownership rules were followed before execution

Languages and Libraries:
Rust (Edition 2021): The core language used.
Standard Library (std):
 - std::io: Utilized for capturing user input and flushing the terminal output.
 - std::vec: Employed the Vec data structure to store sequence results in heap memory.


# Useful Websites

- [ZetCode](https://zetcode.com/rust/operators-expressions/)
- [The Rust Programing Language (The Book)](https://doc.rust-lang.org/std/)

# Future Work

- Item 1 Use the Fibonacci generator to compute ratios of consecutive terms and see how quickly they converge to the golden ratio.
- Item 2 Plot the Fiibonacci numbers to see what patters they make.
- Item 3 Use ratios to generate rectangles, spirals, or tilings.
