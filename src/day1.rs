use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq, Eq)]
struct Dial {
    direction: String,
    value: i64,
}
impl Dial {
    fn load() -> Result<Vec<Dial>, Box<dyn Error>> {
        let mut res: Vec<Dial> = vec![];
        let contents =
            fs::read_to_string("day1_input.txt").expect("Should have been able to read the file");
        let lines: Vec<&str> = contents.split("\n").collect();
        for line in lines {
            if !line.is_empty() {
                res.push(Dial::parse(line)?);
            }
        }
        Ok(res)
    }

    fn turn(self, count: i64) -> (i64, usize) {
        let mut res = 0;
        let mut position = count;
        if self.direction == "L" {
            position = position - self.value;
            while position < 0 {
                position = position + 100;
            }
            if position == 0 {
                res = res + 1;
            }
        } else {
            position = position + self.value;
            while position > 99 {
                position = position - 100;
            }
            if position == 0 {
                res = res + 1;
            }
        }
        (position, res)
    }

    fn parse(data: &str) -> Result<Dial, std::num::ParseIntError> {
        let mut dial = data.to_string();
        let value = dial.split_off(1);

        Ok(Dial {
            direction: dial,
            value: value.parse::<i64>()?,
        })
    }
}

#[test]
pub fn solve() -> Result<(), Box<dyn Error>> {
    let data = Dial::load();
    let mut count = (50, 0);
    let mut tmp: (i64, usize);
    for dial in data? {
        tmp = dial.turn(count.0);
        count = (tmp.0, tmp.1 + count.1);
    }
    dbg!(count.1);
    Ok(())
}
