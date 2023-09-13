// https://exercism.org/tracks/rust/exercises/anagram

pub fn anagrams<'a>(word: &'a str, candidates: &'a [&'a str]) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    let mut nums: Vec<i32> = vec![0; 26];
    word.chars().for_each(|c| {
        let index = if c.is_ascii_lowercase() {
            c as u8 - b'a'
        } else {
            c as u8 - b'A'
        };
        nums[index as usize] += 1
    });
    let dict = nums;
    for candidate in candidates {
        if candidate.len() != word.len() || candidate.to_lowercase() == word.to_lowercase() {
            continue;
        }
        let mut nums: Vec<i32> = vec![0; 26];
        candidate.chars().for_each(|c| {
            let index = if c.is_ascii_lowercase() {
                c as u8 - b'a'
            } else {
                c as u8 - b'A'
            };
            nums[index as usize] += 1
        });
        if nums == dict {
            result.push(candidate);
        }
    }
    return result;
}
