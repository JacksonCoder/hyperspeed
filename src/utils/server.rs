pub fn find_stream_end_chars(msg: String) -> usize {
    let mut sequential_exclamations = 0;
    for character in msg.chars().rev() {
        if character == '!' {
            sequential_exclamations += 1;
        } else {
            sequential_exclamations = 0;
        }
        if sequential_exclamations >= 3 {
            return msg.find(character).unwrap();
        }
    }
    return 0;
}