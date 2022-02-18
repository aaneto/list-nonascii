fn main() {
    let filename: String = std::env::args().skip(1).take(1).collect();
    let filepath = std::path::Path::new(&filename);

    let text = std::fs::read_to_string(&filepath).unwrap();
    let mut chars = std::collections::HashSet::new();

    for ch in text.chars() {
        if !ch.is_ascii() {
            chars.insert(ch);
        }
    }
    
    for ch in chars {
        let char_num = ch as u32;
        println!("{ch} --> {char_num:0>4x}");
    }
}
