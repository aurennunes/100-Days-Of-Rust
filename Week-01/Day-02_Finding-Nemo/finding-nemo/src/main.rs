fn main() {
    let text = "I am finding Nemo !";
    let mut word_position: usize = 0;

    for (index, word) in text.split(" ").enumerate() {
        if word == "Nemo" {
            word_position = index + 1;
            break;
        }
    }

    if word_position != 0 {
        println!("I found Nemo at {}!", word_position);
    } else {
        println!("I can't find Nemo :(");
    }
}
