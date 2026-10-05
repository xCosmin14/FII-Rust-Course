fn is_prime(n: u64) -> bool {
    if n < 2 {return false;}

    let mut i: u64 = 2;
    while i * i <= n {
        if n % i == 0 {return false;}
        i += 1;
    }
    
    true
}

fn co_prime(mut a: u64, mut b: u64) -> bool {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a == 1
}

fn main() {
    println!("Problema 1");
    let mut num: u64 = 2;

    while num < 100 {
        println!("{}: {}", num, is_prime(num));
        num += 1;
    }

    println!("\nProblema 2");
    num = 1;

    while num < 100 {
        let mut num2 = 1; 
        while num2 < 100 {
            println!("{} and {}, {}", num, num2, co_prime(num, num2));
            num2 += 1;
        }
        num += 1;
    }
}