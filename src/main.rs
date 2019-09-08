use std::fs::File;
use std::io::Read;

fn main() {
   start_game(); 
}


fn start_game() {
    println!("Start playing RustyGallows!");

    let words_list = get_words_from_file("word_pool.txt").expect("Error getting words from file");
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
}