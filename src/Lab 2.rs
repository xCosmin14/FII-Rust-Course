//problema 1
fn add_chars_n(mut s: String, c: char, mut n: i32) -> String {
    while n > 0 {
        s.push(c);
        n -= 1;
    }
    s
}

//problema 2
fn add_chars_n_ref(s: &mut String, c: char, mut n: i32) {
    while n > 0 {
        s.push(c);
        n -= 1;
    }
}

//problema 3
fn add_space(s: &mut String, mut n: usize) {
    while n > 0 {
        s.push(' ');
        n -= 1;
    }
}

fn add_str(s: &mut String, s2: String) {
    s.push_str(&s2);
}

fn add_integer(s: &mut String, mut n: i64) {
    if n == 0 {
        s.push('0');
        return;
    }
    if n < 0 {
        s.push('-');
        n *= -1;
    }

    let mut cifre = [0u8; 32];
    let mut lun = 0;
    let mut temp = n;

    while temp > 0 {
        cifre[lun] = (temp % 10) as u8;
        temp /= 10;
        lun += 1;
    }
    for i in (0..lun).rev() {
        s.push((cifre[i] + b'0') as char);
        if i > 0 && i % 3 == 0 {
            s.push('_');
        }
    }
}

fn add_float(s: &mut String, val: f64) {
    let int_part = val as i64;
    add_integer(s, int_part);
    s.push('.');

    let frac_part = (val - int_part as f64).abs();
    let frac_scaled = (frac_part * 1000.0).round() as i64;
    
    let mut cifre = [0u8; 3];
    let mut temp = frac_scaled;

    for i in (0..3).rev() {
        cifre[i] = (temp % 10) as u8;
        temp /= 10;
    }
    for i in 0..3 {
        s.push((cifre[i] + b'0') as char);
    }
}

fn main() {
    // problemele 1-2
    let mut s = String::from("");
    let mut j: String = String::from("");
    let mut i = 0;

    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s = add_chars_n(s, c, 26 - i);
        add_chars_n_ref(&mut j, c, 26 - i);
        i += 1;
    }

    // problema 3 
    let mut r1 = String::from("");
    let mut r2: String = String::from("");

    add_str(&mut r1, String::from("Most "));
    add_space(&mut r2, String::from("Most").len() + 1);
    
    add_str(&mut r2, String::from("downloaded "));
    add_space(&mut r1, String::from("downloaded").len() + 1);
    
    add_str(&mut r1, String::from("crate "));
    add_space(&mut r2, String::from("crate").len() + 1);

    add_str(&mut r2, String::from("has "));
    add_space(&mut r1, String::from("has").len() + 1);

    add_integer(&mut r1, 306437968);
    add_space(&mut r1, 1);
    add_space(&mut r2, String::from("306_437_968").len() + 1);

    add_str(&mut r2, String::from("downloads "));
    add_space(&mut r1, String::from("downloads").len() + 1);

    add_str(&mut r1, String::from("and "));
    add_space(&mut r2, String::from("and").len() + 1);
    
    add_str(&mut r2, String::from("the "));
    add_space(&mut r1, String::from("the").len() + 1);
    
    add_str(&mut r1, String::from("latest "));
    add_space(&mut r2, String::from("latest").len() + 1);

    add_str(&mut r2, String::from("version "));
    add_space(&mut r1, String::from("version").len() + 1);

    add_str(&mut r1, String::from("is "));
    add_space(&mut r2, String::from("is").len() + 1);

    add_float(&mut r2, 2.038);
    add_space(&mut r1, 2);
    add_space(&mut r1, String::from("2.038").len() + 1);

    add_str(&mut r2, String::from("."));

    let non_space_count = r2.chars().filter(|&c| c != ' ').count();

    let mut r3 = String::from("");
    let mut r4 = String::from("");

    add_space(&mut r3, non_space_count);
    add_str(&mut r3, String::from("I "));
    add_str(&mut r3, String::from("💚"));

    add_space(&mut r4, non_space_count);
    add_str(&mut r4, String::from("RUST."));

    println!("{}", r3);
    println!("{}", r4);
    println!("{}", r1);
    println!("{}", r2);
}
