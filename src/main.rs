use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::mem::size_of;

fn main() {
    // // Guessing game
    // let secret = rand::thread_rng().gen_range(1, 101);
    //
    // loop {
    //     println!("Guess some number : ");
    //     let mut guess = String::new();
    //
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");
    //     // .expect method for stop when error occurred
    //
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => {
    //             println!("=== Error : Please Enter a number. ===");
    //             continue;
    //         }
    //     };
    //
    //     match guess.cmp(&secret) {
    //         Ordering::Less => println!("Too small"),
    //         Ordering::Greater => println!("Too big"),
    //         Ordering::Equal => {
    //             println!("Right! You win!!");
    //             break;
    //         }
    //     }
    // }

    // Shadowing variable
    // let x = 5;
    // println!("{}", &x);
    //
    // // Can shadowing using 'let' keyword. same name and variable types of value
    // let x = x + 1;
    // println!("{}", &x);
    //
    // let x = "string";
    // println!("{}", &x);

    // variable types
    // Scalar types
    const BITS_PER_BYTE: usize = 8;
    println!(
        "Your machine is {} bits architecture",
        size_of::<isize>() * BITS_PER_BYTE
    );

    // Rust's character type represent Unicode Scalar type. it's represend over normal ASCII values
    let c: char = '😻';
    println!("{}", c);

    // Tuple.

    let t: (i32, f64, u8) = (500, 1.4, 4);
    println!("{} {} {}", t.0, t.1, t.2);

    // Tuple can disconstruction
    let (x, y, z) = t;
    println!("{} {} {}", x, y, z);

    // Array and vector.
    // Array is Fixed length, using stack with bunch of memory
    // vetor is dynamic langth cuz' using heap area

    let a = [1, 2, 3, 4];
    println!("{}", a[0]); // error occured when runtime. *panic*

    // function
    let str: &str = "String is here";
    println!("str length : {} characters", another_function(str));

    println!("while statement test");
    let mut number = 10;
    while number > 0 {
        println!("{}", number);
        number = number - 1;
    }

    println!("for statement test");
    for n in (1..11).rev() {
        // .rev() is reverse Range
        println!("{}", n);
    }

    println!("for statement with collection iterator test");
    let arr = [1, 2, 3, 4, 5];
    for n in arr.iter() {
        println!("{}", n);
    }
}

// funciton name should be snake case with small case for convention
fn another_function(str: &str) -> usize {
    str.len()
}
