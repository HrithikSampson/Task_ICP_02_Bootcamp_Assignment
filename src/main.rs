use std::io;
struct WordCounter {
    text: String,
}

impl WordCounter {
    pub fn new(text: String) -> Self {
        WordCounter {
            text
        }
    }
    pub fn count_words(&self) -> usize {
        self.text.len() - 2
    }
}

fn main() {
    let mut inp: String = String::new();
    println!("Please Enter a text input!");
    io::stdin().read_line(&mut inp).expect("Error while reading input");
    if inp.len() == 0 {
        panic!("Please Enter valid Input");
    }
    let w = WordCounter::new(inp);
    println!("{}",w.count_words());
}
