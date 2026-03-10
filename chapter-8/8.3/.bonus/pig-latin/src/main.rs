fn main() {

    let mut str = String::from("apple first ٱلسَّلَامُ");

    println!("{str}");

    let mut word_vec: Vec<String> = Vec::new();

    for word in str.split_whitespace() {
        word_vec.push(word.to_string());
    }

    for word in &mut word_vec {
        let str_ref = & mut str;
        let first_char = word.chars().nth(0).unwrap();
        if !is_vowel(&first_char) { 
            let word_cpy = String::from(word.as_str());
            word.remove(0);
            word.push('-');
            word.push(first_char);
            word.push_str("ay");
            str_ref.replace_range(.., str_ref.replace(word_cpy.as_str(), word.as_str()).as_str());
        } else {
            let word_cpy = String::from(word.as_str());
            word.push_str("-hay");
            str_ref.replace_range(.., str_ref.replace(word_cpy.as_str(), word.as_str()).as_str());
        }
    }

    println!("{str}");
}

fn is_vowel(c: &char) -> bool {
    *c == 'a' || *c == 'e' || *c == 'i' || *c == 'o' || *c == 'u'
}
