use std::error::Error;
use std::fs;

pub fn spread(id_string: &str) -> Result<Vec<String>, std::num::ParseIntError> {
    let mut res: Vec<String> = vec![];
    let split: (&str, &str) = id_string.split_once("-").expect("must be '-' there");
    let first: u128 = split.0.trim().parse::<u128>()?;
    let sec: u128 = split.1.trim().parse::<u128>()?;
    for n in first..=sec {
        res.push(n.to_string());
    }
    Ok(res)
}

pub fn is_invalid(id: &str) -> bool {
    let vec: Vec<char> = id.chars().collect();

    if vec.is_empty() || vec.len() % 2 != 0 {
        return false;
    }

    let half = vec.len() / 2;
    let a = &vec[..half];
    let b = &vec[half..];

    a == b
}

pub fn load() -> Result<Vec<String>, Box<dyn Error>> {
    let contents =
        fs::read_to_string("day2_input.txt").expect("Should have been able to read the file");
    let res: Vec<String> = contents.split(',').map(|f| f.trim().to_string()).collect();
    Ok(res)
}

#[test]
fn solve() -> Result<(), Box<dyn Error>> {
    let mut count: u128 = 0;
    let data = load()?;
    for line in data {
        let ids = spread(&line)?;
        for id in ids {
            if is_invalid(&id) {
                count = count + id.parse::<u128>()?;
            }
        }
    }
    dbg!(count);

    Ok(())
}
