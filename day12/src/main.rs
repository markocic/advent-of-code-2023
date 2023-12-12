use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("Could not read from file.");

    let result = process(contents);
    println!("{}", result);
}

fn process(contents: String) -> usize {
    let spring_setup = parse_input(contents);
    println!("{:?}", spring_setup);

    spring_setup.iter().map(|i| i.number_of_combinations()).sum()
}

fn parse_input(contents: String) -> Vec<Springs> {
    let mut result : Vec<Springs> = Vec::new();

    for line in contents.split("\n") {
        let split : Vec<&str> = line.split(" ").collect();
        let damaged = split[1]
                .split(",")
                .filter_map(|i| i.parse().ok())
                .collect::<Vec<usize>>();

        let swaps_left = damaged.iter().sum::<usize>() - split[0].chars().filter(|i| *i == '?').count();
        result.push(Springs {
            record: split[0].to_string(),
            damaged,
            swaps_left
        })
    }

    result
}

fn is_valid(record: String, damaged: Vec<usize>) -> bool {
    
    let record_count : Vec<usize> = record.split(".")
        .filter(|i| !i.is_empty() && !i.contains("?"))
        .map(|i| i.len())
        .collect();

    println!("Checking {}, it's {}", record, record_count == damaged);
    record_count == damaged
}

#[derive(Debug, Clone)]
struct Springs {
    record: String,
    damaged: Vec<usize>,
    swaps_left: usize
}

impl Springs {

    fn number_of_combinations(&self) -> usize {
        self.compute(&mut self.record.to_string(), 0, self.swaps_left)
    }

    fn compute(&self, record: &mut String, curr: usize, swaps_left: usize) -> usize {
        if curr >= record.len() { return 0 }
        let cc = record.chars().nth(curr).unwrap();

        if swaps_left == 0 {
            if is_valid(record.to_owned(), self.damaged.clone()) { return 1; }
            else { return 0; }
        }

        let mut res = 0;

        if cc == '.' { self.compute(record, curr + 1, swaps_left); }

        let mut first : Vec<char> = record.chars().collect();
        first[curr] = '.';
        let mut first_str: String = first.iter().collect();
        res += self.compute(&mut first_str, curr + 1, swaps_left);

        let mut second : Vec<char> = record.chars().collect();
        second[curr] = '#';
        let mut second_str: String = first.iter().collect();

        res += self.compute(&mut second_str, curr + 1, swaps_left - 1);

        res


        // if curr >= record.len() { return 0 }
        //
        // let cc = record.chars().nth(curr).unwrap();
        //
        // if cc != '?' { return 0 }
        //
        //
        // if question_marks > 0 {
        //     let record_pound = record.replace_range(curr..curr+1, "#");
        // } else {
        //
        // }
        //
        // let record_one = record.replace_range(curr..curr+1, ".");
        // 
        // 0
    }
}


#[cfg(test)]
mod test {
    use std::iter::zip;

    use crate::{process, is_valid};

    #[ignore]
    #[test]
    fn test() {
        let t = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1".to_string();

        assert_eq!(process(t), 21);
    }

    #[test]
    fn simple_test() {
        let t = "\
???.### 1,1,3".to_string();
        assert_eq!(process(t), 1);
    }

    #[test]
    fn valid_test() {
        let tests : Vec<String> = vec![
            "#.#.###".to_string(),
            ".#...#....###.".to_string(),
            ".#.###.#.######".to_string(),
            "####.#...#...".to_string(),
            "#....######..#####.".to_string(),
            ".###.##....#".to_string()
        ];

        let damaged : Vec<Vec<usize>> = vec![
            vec![1, 1, 3],
            vec![1, 1, 3],
            vec![1, 3, 1, 6],
            vec![4, 1, 1],
            vec![1, 6, 5],
            vec![3, 2, 1]
        ];

        for (t, d) in zip(tests, damaged) {
            assert_eq!(is_valid(t, d), true);
        }
    }
}
