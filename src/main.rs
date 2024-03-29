use std::fs::File;
use std::io::Read;
use rand::Rng;

fn main() {
   start_game(); 
}


fn start_game() {
    println!("Start playing RustyGallows!");

    let word_pool = get_words_from_file("word_pool.txt").expect("Error getting words from file");
    let word = select_word_from_pool(&word_pool);
    let display_word = get_display_format_from_word(&word);
}

fn get_words_from_file(file_name: &str) -> std::io::Result<(Vec<String>)> {
    let mut file = File::open(file_name)?;
    let mut file_contents = String::new();
    let mut words: Vec<String> = Vec::new();

    file.read_to_string(&mut file_contents)?;

    for string in file_contents.lines() {
        words.push(String::from(string));
    }

    Ok(words)    
}

fn select_word_from_pool(word_pool: &Vec<String>) -> &String {
    let word_index = rand::thread_rng().gen_range(0, word_pool.len());
    word_pool.get(word_index).unwrap()
}

fn get_display_format_from_word(word: &String) -> String {
    let mut result = String::new();

    for character in word.chars() {
        if character == ' ' {
            result.push(' ');
        }
        else {
            result.push('-');
        }
    }

    result
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::Write;
    use super::*;
    use tempfile::tempdir;
    #[test]
    fn test_get_words_from_file() {
        let dir = tempdir().expect("Error creating directory");
        let file_path = dir.path().join("test_file.txt");
        let mut file = File::create(&file_path).expect("Error creating file");
        
        writeln!(file, "testLine1\ntestLine2").expect("Error writing to file");

        let words_list = get_words_from_file(&file_path.to_str().expect("Error getting file path as string")).expect("Error getting word list");

        assert_eq!(words_list[0],"testLine1");
        assert_eq!(words_list[1],"testLine2");
    }

    #[test]
    fn test_select_word_from_pool() {
        let test_pool = vec![String::from("test element")];

        let result = select_word_from_pool(&test_pool);

        assert_eq!(result, "test element");
    }

    #[test]
    fn test_get_display_format_from_word() {
        let word = String::from("test word");
        let result = get_display_format_from_word(&word);

        assert_eq!(result, "---- ----");
    }
}