use rand::Rng;
use std::cmp::Ordering;


#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}


fn main() {

    let point = Point { x: 0, y: 0 };
    
    println!("The point is at {:?}", point);
    // let mut rng = rand::thread_rng();
    // let random_number = rng.gen_range(0, 100);
    // let mut guess: String = String::from("");

    // loop {
    //     guess.clear();
    //     println!("Enter your guess : ");
    //     std::io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Fail to read line");

    //     let number: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     match number.cmp(&random_number) {
    //         Ordering::Less => println!("Too small."),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("It's Right!");
    //             break;
    //         }
    //     }
    // }
}

