struct Note {
    title: String,
    body: String,
}

fn count_char(s: &str, c: char) -> usize {
    let mut appearances: usize = 0;

    for ch in s.chars() {
        if ch == c {
            appearances += 1;
        }
    }

    appearances
}

fn first_word(s: &str) -> &str {
    let mut i = 0;
    for ch in s.chars() {
        if ch == ' ' {
            break;
        }
        i += 1;
    }

    &s[..i]
}

fn reverse(s: &str) -> String {
    let mut new = String::from("");
    for char in s.chars() {
        new.insert(0, char);
    }

    new
}

fn is_palindrome(s: &str) -> bool {
    s == reverse(s)
}

fn repeat_str(s: &str, n: usize) -> String {
    let mut new = String::from("");
    let mut i: usize = 0;

    while i < n {
        new += s;
        i += 1;
    }

    new
}

// ---------- Part 2: mutable borrow (&mut String) ----------

fn add_tag(s: &mut String, tag: &str) {
    s.push('[');
    s.push_str(tag);
    s.push(']');
}

fn censor(s: &mut String, c: char) {
    let mut new = String::new();

    for ch in s.chars() {
        if ch == c {
            new.push('*');
        } else {
            new.push(ch);
        }
    }

    *s = new;
}

// ---------- Part 3: ownership in, ownership out (String) ----------

fn shout(mut s: String) -> String {
    s.make_ascii_uppercase();
    s
}

fn longer_of(a: String, b: String) -> String {
    if a.len() > b.len() {
        return a;
    }

    b
}

// ---------- Part 4: Notebook (a struct that owns its strings) ----------

fn make_note(title: &str, body: &str) -> Note {
    Note {
        title: String::from(title),
        body: String::from(body),
    }
}

fn word_count(note: &Note) -> usize {
    let mut words: usize = 0;

    for ch in note.body.chars() {
        if ch == ' ' && !note.body.is_empty() {
            words += 1;
        }
    }

    words += 1;
    words
}

fn append(note: &mut Note, text: &str) {
    if note.body.is_empty() {
        note.body += text;
    } else {
        note.body += " ";
        note.body += text;
    }
}

fn archive(note: Note) -> String {
    let mut archived_str = String::from("[ARCHIVED]");
    archived_str += " ";
    archived_str += &note.title;
    archived_str += ": ";
    archived_str += &note.body;

    archived_str
}

fn merge(a: Note, b: &Note) -> Note {
    let mut a = a;

    a.title.push('+');
    a.title.push_str(&b.title);

    a.body.push(' ');
    a.body.push_str(&b.body);

    a
}

fn main() {
    // Part 1
    let text = String::from("level up your rust");
    println!("{}", count_char(&text, 'e'));
    println!("{}", count_char("banana", 'a'));
    println!("{}", first_word(&text));
    println!("{}", first_word("rust"));
    println!("{}", reverse(&text));
    println!("{} {}", is_palindrome("level"), is_palindrome(&text));
    println!("{}", repeat_str("ab", 3));

    // // Part 2
    let mut s = String::from("hello world");
    add_tag(&mut s, "v2");
    println!("{s}");
    censor(&mut s, 'o');
    println!("{s}");

    // // Part 3
    let loud = shout(s);
    println!("{loud}");
    let winner = longer_of(String::from("tiny"), loud);
    println!("{winner}");

    // // Part 4
    let title = String::from("shopping");
    let mut n1 = make_note(&title, "milk");
    let n2 = make_note("ideas", "learn rust borrowing");
    println!("{title}");
    println!("{} word(s)", word_count(&n1));
    append(&mut n1, "eggs");
    append(&mut n1, "bread");
    println!("{}: {}", n1.title, n1.body);
    let merged = merge(n1, &n2);
    println!("{}: {}", merged.title, merged.body);
    println!("{} word(s)", word_count(&merged));
    println!("{}", archive(n2));
}
