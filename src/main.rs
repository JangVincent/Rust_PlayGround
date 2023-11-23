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

    // Ownership with funciton
    let sample_string = String::from("qweruiop");
    let (sample_string, string_length) = calc_string(sample_string);
    println!("{} {}", sample_string, string_length);

    // 두 상황중 하나만 가능함
    // 하나의 가변 참조자 (&mut)
    // 여러개의 불변 참조자 (&)
    let mut ss = String::from("hello");
    let r2 = &ss;
    let r1 = &mut ss;

    // Slice
    let mut test = String::from("Test is important");
    let word = first_word_index(&test);
    test.clear(); // 문자열을 빈 문자열로 만듦
    println!("{}", word); // word 값은 변하지 않음 사실상 word 는 문자열과 상관 없는 값이 되버림.

    let test2 = String::from("Test is important");
    let slice1 = &test2[0..test2.len()]; // test2 의 일부 참조자. end 를 포함하지 않음
    let slice2 = &test2[..2]; // start from 0
    let total_slice = &test[..];

    println!("{}", slice1);

    // let mut t = String::from("Test is important");
    // let word = first_word(&t); // immutable reference here.
    // t.clear(); // Compile error : mutable reference here
    // println!("{}", word);

    // String literal is slice
    let asdf = "Hello, world!";
    // asdf type is &str(slice). it pointer specific binary position.
    // 따라서 스트링 리터럴은 불변참조자임.

    // slice can use with array
    let a = [1, 2, 3, 4, 5];
    let a_slice = &a[1..3]; // type : &[i32]
}

// funciton name should be snake case with small case for convention
fn another_function(str: &str) -> usize {
    str.len()
}

fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

fn calc_string(s: String) -> (String, usize) {
    // tuple 에 moved 된 값(heap 을 사용하는 값은 함수의 인수로 넘어가면 moved 됨.)을 사용해서 여러 원소에 넣으려고 하면,
    // 이미 moved 된 값은 borrow 할 수 없다는 경고가 뜸
    // ex) (s, s.len) // 튜플의 두번째 원소 부터는 s 를 직접 사용하려고 하거나 method 등으로 연산할 때 borrow 가 일어남.
    // 이미 첫 원소에서 s 를 사용해버리면서 소유권이 tuple 로 moved 된다고 판단 하는 듯 하다.

    let len = s.len();
    (s, len)
}

// 댕글링 포인터이면 안됨
// fn dangle() -> &String {
//     let s = String::from("hello")
//     s
// }

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &String) -> &str {
    //using string slice
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// 이렇게 함수를 만들면 String 형에 대한 전체 slice 와 &str 모두 전달 가능.
fn refactor_first_word(s: &str) -> &str {
    // todo something
    "asdf"
}
