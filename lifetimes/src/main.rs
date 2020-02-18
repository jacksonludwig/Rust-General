fn main() {
    let r;

   // {
        let x = 5;
        r = &x;
  //  }

    println!("r: {}", r);


    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str()/* slice */, string2);
    println!("The longest string is {}\n", result);

    // --------------------------
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    // --------------------------

    let s: &'static str = "I have a static lifetime.";
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// -----------------------------
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}