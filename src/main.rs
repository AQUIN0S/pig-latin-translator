use ::pig_latin::translate_word;

fn main() {
    let test_words = ["hello", "bye", "Testing", "which", "rhythm", "enter"];
    
    for word in test_words.iter() {
        println!("The pig latin equivalent of {} is {}", word, translate_word(String::from(*word)))
    }
}
