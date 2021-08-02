use slices::first_word;
fn main() {
    let s = String::from("Hello World!");
    let word_index = first_word(&s);
    println!("{}", word_index);
    let my_string_literal = "My new world!!";
    let word = first_word(my_string_literal);
    println!("{}", word);

}
