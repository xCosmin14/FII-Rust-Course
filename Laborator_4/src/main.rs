use std::num::ParseIntError;

//<problema 1>
fn next_prime(x: u16) -> Option<u16> {
    if x == u16::MAX {
        return None;
    }

    let mut x2 = x + 1;
    if x2.is_multiple_of(2) {
        x2 += 1;
    }

    while x2 < u16::MAX {
        let mut i: u16 = 2;
        let mut is_prime: bool = true;

        while i * i <= x2 {
            if x2.is_multiple_of(i) {
                is_prime = false;
                break;
            }
            i += 1;
        }

        if is_prime {
            return Some(x2);
        }

        x2 += 2;
    }

    None
}
//</problema 1>

//<problema 2>
fn check_addition(x: u32, y: u32) -> u32 {
    match x.checked_add(y) {
        Some(val) => val,
        None => panic!("Sum is greater than u32 limit"),
    }
}

fn check_multiplication(x: u32, y: u32) -> u32 {
    match x.checked_mul(y) {
        Some(val) => val,
        None => panic!("Product is greater than u32 limit"),
    }
}
//</problema 2>

//<problema 3>
#[derive(Debug)]
enum MathError {
    Overflow,
}

fn check_addition_result(x: u32, y: u32) -> Result<u32, MathError> {
    x.checked_add(y).ok_or(MathError::Overflow)
}

fn check_multiplication_result(x: u32, y: u32) -> Result<u32, MathError> {
    x.checked_mul(y).ok_or(MathError::Overflow)
}

fn calculate_complex_math(x: u32, y: u32, z: u32) -> Result<u32, MathError> {
    let sum = check_addition_result(x, y)?;
    let product = check_multiplication_result(sum, z)?;
    Ok(product)
}
//</problema 3>

//<problema 4>
#[derive(PartialEq)]
enum MyError {
    IsNotAscii,
    IsNotDigit,
    IsNotBase16Digit,
    IsNotLetter,
    IsNotPrintable,
}

fn to_uppercase(c: char) -> Result<char, MyError> {
    if !c.is_ascii() {
        return Err(MyError::IsNotAscii);
    }
    if !c.is_ascii_alphabetic() {
        return Err(MyError::IsNotLetter);
    }

    Ok(c.to_ascii_uppercase())
}

fn to_lowercase(c: char) -> Result<char, MyError> {
    if !c.is_ascii() {
        return Err(MyError::IsNotAscii);
    }
    if !c.is_ascii_alphabetic() {
        return Err(MyError::IsNotLetter);
    }

    Ok(c.to_ascii_lowercase())
}

fn print_char(c: char) -> Result<(), MyError> {
    if !c.is_ascii() {
        return Err(MyError::IsNotAscii);
    }
    if !c.is_ascii_graphic() && c != ' ' {
        return Err(MyError::IsNotPrintable);
    }

    println!("{c}");
    Ok(())
}

fn char_to_number(c: char) -> Result<u32, MyError> {
    if !c.is_ascii() {
        return Err(MyError::IsNotAscii);
    }
    if !c.is_ascii_digit() {
        return Err(MyError::IsNotDigit);
    }

    Ok(c.to_digit(10).unwrap())
}

fn char_to_number_hex(c: char) -> Result<u32, MyError> {
    if !c.is_ascii() {
        return Err(MyError::IsNotAscii);
    }
    if !c.is_ascii_hexdigit() {
        return Err(MyError::IsNotBase16Digit);
    }

    Ok(c.to_digit(16).unwrap())
}

fn print_error(error: MyError) {
    match error {
        MyError::IsNotAscii => println!("Eroare: Caracterul nu este ASCII."),
        MyError::IsNotDigit => println!("Eroare: Caracterul nu este o cifră."),
        MyError::IsNotBase16Digit => {
            println!("Eroare: Caracterul nu este o cifră validă în baza 16.")
        }
        MyError::IsNotLetter => println!("Eroare: Caracterul nu este o literă."),
        MyError::IsNotPrintable => println!("Eroare: Caracterul nu este printabil."),
    }
}
//</problema 4>

//<problema 5>
fn parse_age(input: &str) -> Result<u8, ParseIntError> {
    input.trim().parse::<u8>()
}

fn get_discount_code(age: u8) -> Option<String> {
    match age {
        0..=12 => Some(String::from("KIDS_HALF_PRICE")),
        65..=150 => Some(String::from("SENIOR_DISCOUNT_30")),
        _ => None,
    }
}
//</problema 5>

fn main() {
    //<problema 1>
    let mut current = u16::MIN;

    let mut count = 0;
    while let Some(p) = next_prime(current) {
        println!("{}", p);
        current = p;
        count += 1;
        if count >= 10 {
            break;
        }
    }
    //</problema 1>

    let x1: u32 = 4000;
    let y1: u32 = 5000;
    let x2: u32 = 4294967295;
    let y2: u32 = 4294967295;

    //<problema 2>
    println!("x1+y1 (succes): {}", check_addition(x1, y1));
    println!("x1*y1 (succes): {}", check_multiplication(x1, y1));

    println!("x2+y2 (panic): {}", check_addition(x2, y2));
    println!("x2*y2 (panic): {}", check_multiplication(x2, y2));
    //</problema 2>

    //<problema 3>
    println!("\nTestare Problema 3 (Result):");

    match calculate_complex_math(x1, y1, 2) {
        Ok(result) => println!("(x1 + y1) * 2 = {}", result),
        Err(e) => println!("Calculus error: {:?}", e),
    }

    match calculate_complex_math(x2, y2, 2) {
        Ok(result) => println!("(x2 + y2) * 2 = {}", result),
        Err(e) => println!("Calculus error: {:?}", e),
    }
    //</problema 3>

    //<problema 4>
    match char_to_number('A') {
        Ok(num) => println!("Number is: {}", num),
        Err(e) => print_error(e),
    }

    match char_to_number_hex('F') {
        Ok(num) => println!("Hex number is: {}", num),
        Err(e) => print_error(e),
    }

    println!("\n--- Testare to_uppercase ---");
    let test_chars_upper = ['a', 'Z', '5', 'ș'];
    for &c in &test_chars_upper {
        match to_uppercase(c) {
            Ok(upper) => println!("Char '{}' transformed: '{}'", c, upper),
            Err(e) => {
                print!("Error for '{}': ", c);
                print_error(e);
            }
        }
    }

    println!("\n--- Testare to_lowercase ---");
    let test_chars_lower = ['B', 'y', '!', 'ț'];
    for &c in &test_chars_lower {
        match to_lowercase(c) {
            Ok(lower) => println!("Character '{}' transformed: '{}'", c, lower),
            Err(e) => {
                print!("Error for '{}': ", c);
                print_error(e);
            }
        }
    }

    println!("\n--- Testare print_char ---");
    let test_chars_print = ['X', ' ', '\n', 'ă'];
    for &c in &test_chars_print {
        print!("Testing print_char on '{}': ", c.escape_debug());
        match print_char(c) {
            Ok(_) => (),
            Err(e) => print_error(e),
        }
    }
    //</problema 4>

    //<problema 5>
    let user_inputs = vec!["10", "32", "75", "invalid_age"];

    for input in user_inputs {
        println!("Processing input: '{}'", input);

        match parse_age(input) {
            Ok(age) => match get_discount_code(age) {
                Some(code) => println!("  Success! User gets code: {}", code),
                None => println!(
                    "  Success! User is {} years old, but no discount applies.",
                    age
                ),
            },
            Err(e) => {
                println!("  Error: Could not parse age. Reason: {}", e);
            }
        }
        println!("---");
    }
    //</problema 5>
}
