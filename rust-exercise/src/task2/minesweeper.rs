// use std::io::{self, Read};

static NEIGHBORS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    // (0, 0),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn sweep(input: &[&str]) -> Vec<Vec<char>> {
    // get the size of the map
    let row = input.len();
    let col = input[0].chars().count();
    let result = input
        .iter()
        .enumerate()
        .flat_map(|(row_index, line)| {
            line.chars()
                .enumerate()
                .map(|(col_index, c)| match c {
                    '·' => {
                        let mut count = 0;
                        for (i, j) in NEIGHBORS.iter() {
                            let x = row_index as i32 + i;
                            let y = col_index as i32 + j;
                            if x < 0 || x >= row as i32 || y < 0 || y >= col as i32 {
                                continue;
                            }
                            let neighbor = input[x as usize].chars().nth(y as usize).unwrap();
                            if neighbor == '*' {
                                count += 1;
                            }
                        }
                        if count == 0 {
                            '·'
                        } else {
                            std::char::from_digit(count, 10).unwrap()
                        }
                    }
                    _ => c,
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<char>>()
        .chunks(col)
        .map(|x| x.to_vec())
        .collect::<Vec<Vec<char>>>();
    return result;
}
