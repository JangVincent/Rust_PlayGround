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

    // Variable types
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

    // Function
    let str: &str = "String is here";
    println!("str length : {} characters", another_function(str));

    println!("while statement test");
    let mut number = 10;
    while number > 0 {
        println!("{}", number);
        number = number - 1;
    }

    println!("'For' statement test");
    for n in (1..11).rev() {
        // .rev() is reverse Range
        println!("{}", n);
    }

    println!("'For' statement with collection iterator test");
    let arr = [1, 2, 3, 4, 5];
    for n in arr.iter() {
        println!("{}", n);
    }

    // Type of function implement
    println!("Typeof test");
    let x = 3;
    println!("{}", type_of(&x));

    // Ownership
    // String type can append, but string literal is not.
    // Cuz, String literal value can specified in compile time, so hard-coded in final executable file
    println!("Ownership test :: String");
    let mut s = String::from("Hello rust wold!");
    s.push_str(", made by phantola");
    println!("{}", s);

    {
        #[warn(unused_variables)]
        let a = String::from("hello this is local string");
        // something do with a
    }
    // vairable 'a' is not more usable. Rust free memory when meet '}' automatically

    // 러스트는 아래와 같이 기존의 heap 을 사용하는 변수를 다른 변수에 할당하게 되면,
    // 복사가 아니라 move (이동) 이 일어난다. 따라서 s 를 출력하려고 하면 아예 동작하지 않고
    // 에러뱉음.
    // let s2 = s; 하는 순간, s의 포인터가 무효화되고, s2 의 포인터가 바인딩 되기 때문이다.
    let s2 = s;
    // Error occurred!
    // print!("{}", s);

    // 만약 정말 deep copy 하고 싶다면,
    // 이렇게 .clone() method 사용해야함
    let s3 = s2.clone();
    println!("{}", s3);

    // Normal stackable variable (int, float...) has 'Copy' Trait.
    // If some type has Copy trait, it can be use after re-assign like downward
    let a = 3;
    let b = a;
    println!("{}, {}", a, b); // OK
}

// funciton name should be snake case with small case for convention
fn another_function(str: &str) -> usize {
    str.len()
}

fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}
