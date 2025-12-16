use std::io;

//Use of references (&mut) and mutable variables.
fn calculate_fibonacci(sequence: &mut Vec<u64>, count: u32) {
    // Unmutable variable to hold the limit
    let limit: u32 = count;
    
    let mut term_zero: u64 = 0;
    let mut term_one: u64 = 1;
    
    let mut loop_counter: u32 = 0;

    while loop_counter < limit {
        // Conditionals: Handle the first two special cases of the sequence
        if loop_counter == 0 {
            // Expression: Pushing the initial value onto the Vec
            sequence.push(term_zero);
        } else if loop_counter == 1 {
            // Expression: Pushing the second initial value onto the Vec
            sequence.push(term_one);
        } else {
            // Expression: Calculate the next Fibonacci number
            let next_fib: u64 = term_zero + term_one; 
            
            // Expression: Update 'term_zero' and ' term_one' for the next iteration
            term_zero = term_one;
            term_one = next_fib;

            // Expression: Add the new number to the sequence
            sequence.push(term_one); 
        }

        loop_counter = loop_counter + 1;
    }
}


fn find_primes_in(sequence: &Vec<u64>) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::new();
    
    for number in sequence {
        if is_prime(*number){
            primes.push(*number);
        }
    }
    primes
}

// Checks if a number is a sequence is prime
fn is_prime(number: u64) -> bool {
    if number < 2 {
        return false;
    }
    else if number == 2 {
        return true;
    }
    else if number % 2 == 0 {
        return false;
    }    


    let mut odd_diviser = 3;

    while odd_diviser * odd_diviser <= number {
        if number % odd_diviser == 0 {
            return false;
        }
        odd_diviser += 2;
    }
    true
}

   
fn slice_first_five_of(sequence: &Vec<u64>) -> () {
    if sequence.len() > 5 {
        let slice: &[u64] = &sequence[..5];
        println!("The first 5 terms (using slicing): {:?}", slice);
    } else {
        println!("Not enough terms to slice beyond the full sequence.");
    }
}


fn slice_last_five_of(sequence: &Vec<u64>) -> () {
    if sequence.len() > 5 {
        let slice: &[u64] = &sequence[sequence.len() - 5..];
        println!("The last 5 terms (using slicing): {:?}", slice);
    } else {
        println!("Not enough terms to slice beyond the full sequence.");
    }
}
    

fn main() {
    println!("🔢 **Fibonacci Sequence Generator**");
    println!("Enter the number of terms you want to generate (e.g., 10):");

    // Declare a mutable variable to store user input.
    let mut input = String::new();

    // Use a loop to handle potential input errors (robust error handling)
    let count: u32 = loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // Expression: Trim the whitespace and attempt to parse the string to a u32.
                match input.trim().parse::<u32>() {
                    Ok(num) => break num, // Success: exit the loop with the number
                    Err(_) => {
                        println!("That wasn't a valid number. Please enter a positive integer:");
                        input.clear(); 
                    }
                }
            }
            Err(error) => {
                println!("Error reading line: {}", error);
                input.clear();
            }
        }
    };
    
    // Use a mutable Vec<u64> data structure to store the resulting sequence.
    let mut fib_sequence: Vec<u64> = Vec::new();
    
    calculate_fibonacci(&mut fib_sequence, count);

    println!("\n The first {} Fibonacci numbers are:", count);
    
    // This 'for' loop iterates over the elements of the Vec by reference.
    for number in &fib_sequence {
        print!("{}, ", number);
    }
    println!("\n");

    slice_first_five_of(&fib_sequence);
    slice_last_five_of(&fib_sequence);

    let primes = find_primes_in(&fib_sequence); 
    println!("Primes: {:?}", primes);
}