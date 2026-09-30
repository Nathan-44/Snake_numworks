use alloc::string::ToString;
use crate::nadk::storage;

const FILE_NAME: &str = "best_score.txt";

/// Save score in a file
pub fn save_score(score: u8) {
    let score_string = score.to_string();
    let score_text = score_string.as_str();

    // If the file exist, we erase it
    storage::file_erase(FILE_NAME);

    // We create a new file and we write score
    storage::file_write(FILE_NAME, score_text.as_bytes());
}

pub fn load_score() -> u8 {
    // We try to load file if it exist
    if storage::file_exists(FILE_NAME) {
        let Some(text_u8) = storage::file_read(FILE_NAME) else {
            return 0;
        };
        let Ok(text_str) = str::from_utf8(&text_u8) else {
            return 0;
        };
        if let Ok(score) = text_str.parse::<u8>() { score }
        else { 0 }
    }
    else { return 0; }
}

