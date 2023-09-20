use std::collections::HashMap;
use std::thread;

fn thread_fn(str: &str, hmap: &mut HashMap<char, i32>) {
    for c in str.chars() {
        let count = hmap.get(&c);
        match count {
            Some(x) => {
                hmap.insert(c, x + 1);
            }
            None => {
                hmap.insert(c, 1);
            }
        }
    }
}

pub fn frequency(str: &str) -> HashMap<char, i32> {
    let thread_num = 3;
    let vec_str = str.chars().collect::<Vec<char>>();
    let partition = vec_str.len() / thread_num;
    let remain = vec_str.len() % thread_num;
    let mut hmaps = vec![];
    for i in 0..thread_num {
        let end = if i == thread_num - 1 {
            (i + 1) * partition + remain
        } else {
            (i + 1) * partition
        };
        let str = vec_str[i * partition..end].iter().collect::<String>();
        let handle = thread::spawn(move || {
            let mut tmp_hmap: HashMap<char, i32> = HashMap::new();
            thread_fn(&str, &mut tmp_hmap);
            tmp_hmap
        });
        hmaps.push(handle.join().unwrap());
    }

    let hmap = hmaps.iter().fold(HashMap::new(), |mut target, x| {
        for (k, v) in x.iter() {
            let count = target.get(k);
            match count {
                Some(x) => {
                    target.insert(*k, x + v);
                }
                None => {
                    target.insert(*k, *v);
                }
            }
        }
        target
    });
    return hmap;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concurrency() {
        let answer = frequency("abca");
        println!("{:?}", answer);
        let mut hmap: HashMap<char, i32> = HashMap::new();
        thread_fn("abca", &mut hmap);
        println!("{:?}", hmap);
        assert_eq!(answer, hmap);
    }
}
