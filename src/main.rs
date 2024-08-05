use std::collections::{HashMap, HashSet};

fn main() {
    println!("Welcome to the Rust Learning Game!");

    loop {
        println!("\nChoose an option to learn about:");
        println!("1. Integer Operations");
        println!("2. Floating-Point Operations");
        println!("3. Boolean Operations");
        println!("4. Character Operations");
        println!("5. String Operations");
        println!("6. Collections");
        println!("7. Option and Result");
        println!("8. Exit");

        let mut choice: String = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u8 = match choice.trim().parse() {
            Ok(num) if num >= 1 && num <= 8 => num,
            _ => {
                println!("Invalid choice. Please select a number between 1 and 8.");
                continue;
            }
        };

        match choice {
            1 => integer_operations(),
            2 => floating_point_operations(),
            3 => boolean_operations(),
            4 => char_operations(),
            5 => string_operations(),
            6 => collections(),
            7 => option_result(),
            8 => {
                println!("Exiting the game. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Try again."),
        }
    }
}

fn integer_operations() {
    println!("\n--- Integer Operations ---");

    let a: i32 = 10;
    let b: i32 = 5;
    println!("Variables: a = {}, b = {}", a, b);

    println!("Sum: a + b = {}", a + b);
    println!("Difference: a - b = {}", a - b);
    println!("Product: a * b = {}", a * b);
    println!("Quotient: a / b = {}", a / b);
    println!("Remainder: a % b = {}", a % b);

    let x: i32 = 42;
    let y: u64 = x as u64; // Type casting
    println!("Converted x from i32 to u64: x as u64 = {}", y);

    let c: u8 = 0b00001111;
    let d: u8 = 0b11110000;
    println!("Bitwise AND: c & d = {}", c & d);
    println!("Bitwise OR: c | d = {}", c | d);
    println!("Bitwise XOR: c ^ d = {}", c ^ d);
    println!("Bitwise NOT: !c = {}", !c);

    let result: Option<i32> = x.checked_add(10);
    println!("Checked addition result: x.checked_add(10) = {:?}", result);
}

fn floating_point_operations() {
    println!("\n--- Floating-Point Operations ---");

    let a: f64 = 3.5;
    let b: f64 = 2.0;
    println!("Variables: a = {}, b = {}", a, b);

    println!("Sum: a + b = {}", a + b);
    println!("Difference: a - b = {}", a - b);
    println!("Product: a * b = {}", a * b);
    println!("Quotient: a / b = {}", a / b);

    println!("Square root of a: a.sqrt() = {}", a.sqrt());
    println!("a to the power of b: a.powf(b) = {}", a.powf(b));
    println!("Exponent of a: a.exp() = {}", a.exp());
    println!("Natural log of a: a.ln() = {}", a.ln());
    println!("Sine of a: a.sin() = {}", a.sin());

    println!("Positive infinity: f64::INFINITY = {}", f64::INFINITY);
    println!(
        "Negative infinity: f64::NEG_INFINITY = {}",
        f64::NEG_INFINITY
    );
    println!("NaN: f64::NAN = {}", f64::NAN);
}

fn boolean_operations() {
    println!("\n--- Boolean Operations ---");

    let a: bool = true;
    let b: bool = false;
    println!("Variables: a = {}, b = {}", a, b);

    println!("Logical AND: a && b = {}", a && b);
    println!("Logical OR: a || b = {}", a || b);
    println!("Logical NOT: !a = {}", !a);
}

fn char_operations() {
    println!("\n--- Character Operations ---");

    let letter: char = 'a';
    println!("Character: letter = {}", letter);
    println!(
        "Is alphabetic: letter.is_alphabetic() = {}",
        letter.is_alphabetic()
    );
    println!("Is numeric: letter.is_numeric() = {}", letter.is_numeric());
    println!(
        "Is whitespace: letter.is_whitespace() = {}",
        letter.is_whitespace()
    );
    println!(
        "Uppercase: letter.to_uppercase().collect::<String>() = {}",
        letter.to_uppercase().collect::<String>()
    );
}

fn string_operations() {
    println!("\n--- String Operations ---");

    let mut s: String = String::from("Hello");
    s.push_str(", World!");
    s.push('!');
    println!("Modified string: {}", s);

    let s1: &str = "   Hello   ";
    let trimmed: &str = s1.trim();
    let replaced: String = s1.replace("Hello", "Hi");
    println!("Trimmed: s1.trim() = '{}'", trimmed);
    println!("Replaced: s1.replace(\"Hello\", \"Hi\") = '{}'", replaced);

    let formatted: String = format!("Formatted string: {} {}", s1, trimmed);
    println!(
        "Formatted: format!(\"Formatted string: {} {}\", s1, trimmed) = {}",
        s1, trimmed, formatted
    );

    let slice: &str = &s1[0..5];
    println!("Substring: &s1[0..5] = '{}'", slice);

    let chars: Vec<char> = s1.chars().collect();
    println!(
        "Characters: s1.chars().collect::<Vec<char>>() = {:?}",
        chars
    );

    let lines: Vec<&str> = s1.lines().collect();
    println!("Lines: s1.lines().collect::<Vec<&str>>() = {:?}", lines);
}

fn collections() {
    println!("\n--- Collections ---");

    let mut vec: Vec<i32> = vec![1, 2, 3];
    vec.push(4);
    println!("Vector: vec![1, 2, 3] = {:?}", vec);

    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("key1", 10);
    map.insert("key2", 20);
    println!("HashMap: {:?}", map);
    println!(
        "Value for 'key1': map.get(\"key1\") = {:?}",
        map.get("key1")
    );
    map.remove("key2");
    println!("HashMap after removing 'key2': {:?}", map);

    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    println!("HashSet: {:?}", set);
    println!("Contains 1: set.contains(&1) = {}", set.contains(&1));
    set.remove(&2);
    println!("HashSet after removing 2: {:?}", set);
}

fn option_result() {
    println!("\n--- Option and Result ---");

    let some_value: Option<i32> = Some(5);
    let none_value: Option<i32> = None;
    println!("Some value: Some(5) = {:?}", some_value);
    println!("None value: None = {:?}", none_value);

    let mapped: Option<i32> = some_value.map(|x: i32| x * 2);
    println!(
        "Mapped Some value: some_value.map(|x| x * 2) = {:?}",
        mapped
    );
    println!(
        "Unwrapped or default: some_value.unwrap_or(0) = {}",
        some_value.unwrap_or(0)
    );
    println!("Is Some: some_value.is_some() = {}", some_value.is_some());
    println!("Is None: none_value.is_none() = {}", none_value.is_none());

    let ok_result: Result<i32, &str> = Ok(10);
    let err_result: Result<i32, &str> = Err("error");
    println!("Ok result: Ok(10) = {:?}", ok_result);
    println!("Err result: Err(\"error\") = {:?}", err_result);

    let mapped_result: Result<i32, &str> = ok_result.map(|x| x * 2);
    println!(
        "Mapped Ok result: ok_result.map(|x| x * 2) = {:?}",
        mapped_result
    );
    println!(
        "Unwrapped or default: ok_result.unwrap_or(0) = {}",
        ok_result.unwrap_or(0)
    );
    println!("Is Ok: ok_result.is_ok() = {}", ok_result.is_ok());
    println!("Is Err: err_result.is_err() = {}", err_result.is_err());
}
