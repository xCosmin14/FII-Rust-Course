use std::{fs, time::Instant};

fn longest_line_in_file(path: &str) {
    let textfile_contents = fs::read_to_string(path).unwrap();

    let mut longest_line_by_bytes = String::from("");
    let mut longest_in_bytes: usize = 0;
    let mut longest_line_by_chars = String::from("");
    let mut longest_in_chars: usize = 0;

    for line in textfile_contents.lines() {
        println!(
            "Line -- {line} -- is {} in bytes and {} in chars",
            line.len(),
            line.chars().count()
        );

        if line.len() > longest_in_bytes {
            longest_line_by_bytes = String::from(line);
            longest_in_bytes = line.len();
        }

        if line.chars().count() > longest_in_chars {
            longest_line_by_chars = String::from(line);
            longest_in_chars = line.chars().count();
        }
    }

    println!("Longest line in bytes: {longest_line_by_bytes} - {longest_in_bytes}B");
    println!("Longest line in chars: {longest_line_by_chars} - {longest_in_chars} chars");
}

fn rot13(path: &str) {
    let textfile_contents = fs::read_to_string(path).unwrap();

    for line in textfile_contents.lines() {
        let mut new = String::from("");

        for ch in line.chars() {
            if !ch.is_ascii() {
                println!("Character {ch} is not ASCII. Abandoning operation...");
            }
    
            let rotated = match ch {
                'a'..='z' => (((ch as u8 - b'a' + 13) % 26) + b'a') as char,
                'A'..='Z' => (((ch as u8 - b'A' + 13) % 26) + b'A') as char,
                _ => ch
            };
    
            new.push(rotated);
        }
        println!("{new}");
    }
}

fn replace_abbreviations(abbr_path: &str, text: &str) {
    let abbr_contents: String = fs::read_to_string(abbr_path).unwrap();

    for line in text.lines() {
        let mut new = String::from("");

        for cuvant in line.split(' ') {
            let mut cuvant_inlocuit = cuvant; 

            for abbr_line in abbr_contents.lines() {
                if let Some((cuvant_complet, abreviere)) = abbr_line.split_once(' ') 
                && cuvant == abreviere{
                    cuvant_inlocuit = cuvant_complet;
                    break;    
                }
            }

            if !new.is_empty() {
                new.push(' ');
            }
            new.push_str(cuvant_inlocuit);
        }

        println!("{new}");
    }
}

fn format_hosts(path: &str) {
    let textfile_contents = fs::read_to_string(path).unwrap();
    let mut new = String::from(""); 
    
    for line in textfile_contents.lines() {
        if !line.is_empty() && !line.starts_with('#') {
            
            let mut first_word = true;

            for entry in line.split_whitespace() {
                if !first_word {
                    new.push_str(" => ");
                }
                
                new.push_str(entry);
                first_word = false;
            }
            
            new.push('\n');
        }
    }
    
    println!("{new}");
}

fn rot13_opt(path: &str) -> String {
    let textfile_contents = fs::read_to_string(path).unwrap();

    let mut bytes = textfile_contents.into_bytes();      
    for i in 0..bytes.len() {               
        let b = bytes[i];

        bytes[i] = match b {
            b'a'..=b'm' | b'A'..=b'M' => b + 13,
            b'n'..=b'z' | b'N'..=b'Z' => b - 13,
            _ => b,                          
        };
    }
    String::from_utf8(bytes).unwrap()
}

fn main() {
    //problema 1
    longest_line_in_file("src/inputs1.txt");

    println!();

    //problema 2
    rot13("src/inputs2.txt");

    println!();

    //problema 3
    replace_abbreviations(
        "src/inputs3.txt",
        "sall frt ce mai faci, totul este bn ?
              eu besc sa merg la fac de info
              am lucrat la un pc nou in acel lab
              am fost la dl Matei si dna Maria ptr o discutie"
    );

    println!();

    //problema 4
    format_hosts("C:/Windows/System32/drivers/etc/hosts");

    //bonus 
    let start = Instant::now();
    rot13_opt("src/inputs5.txt");
    println!("{:?}", start.elapsed());
    //problema 2 da 580 microsecunde pe cateva linii, iar optimizarea 140 pe cargo run
    //pe cargo run --release, acelasi text da 710 cu 180, dar timpii sunt mai mari in general oricum
    //pe un text de 400mb, timpul de executie depaseste lejer 30 de secunde pe ambele variante, 
    //nici nu are rost sa tin programul deschis (o sa imi crape laptopul)
    //*git nu ma lasa sa pun fisierul mare .txt asa ca daca trebuie testat, incarca-l ca 
    //inputs5.txt in src* 
}
