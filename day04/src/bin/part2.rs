use std::fmt;
use std::fs;

fn main() {

    // let contents = fs::read_to_string("src/input.txt").expect("Error reading file.");

    let contents = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string();

    let result = process(contents);
    println!("{}", result);
}

fn process(input: String) -> usize {
    
    let cards = parse_cards(input);

    let mut sum = 0;
    let mut counter: usize = 0;
    for card in &cards {
        
    }

    sum
}

fn parse_cards(input: String) -> Vec<Card> {
    let mut result : Vec<Card> = Vec::new();

    let lines = input.split("\n");

    for line in lines {

        if line.len() < 1 { continue; }

        let index = extract_index(line);
        let my_nums = extract_my_nums(line);
        let winning_nums = extract_winning_nums(line);

        result.push(Card {
            index,
            my_numbers: my_nums,
            winning_numbers: winning_nums
        })

    }
    result
}

fn extract_winning_nums(line: &str) -> Vec<usize> {
    line.split("|")
        .collect::<Vec<&str>>()[1]
        .split(" ")
        .filter_map(|i| {
            i.parse::<usize>().ok()
        })
        .collect::<Vec<usize>>()
}

fn extract_my_nums(line: &str) -> Vec<usize> {
   line.split("|")
        .collect::<Vec<&str>>()[0]
        .split(":")
        .collect::<Vec<&str>>()[1]
        .split(" ")
        .filter_map(|i| {
            i.parse::<usize>().ok()
        })
        .collect::<Vec<usize>>()
}

fn extract_index(line: &str) -> usize {
    let parts : Vec<&str> = line.split(":").collect();

    let card_str = parts[0].split(" ").collect::<Vec<&str>>();

    card_str[card_str.len() - 1].parse::<usize>().unwrap()
}


struct Card {
    index: usize,
    my_numbers: Vec<usize>,
    winning_numbers: Vec<usize>,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut my_nums = String::from("");
        for num in &self.my_numbers {
            my_nums.push_str(" ");
            my_nums.push_str(&num.to_string());
        }

        let mut winning_nums = String::from("");
        for num in &self.winning_numbers {
            winning_nums.push_str(" ");
            winning_nums.push_str(&num.to_string());
        }

        write!(f, "Card {}: {} |{}", self.index, my_nums, winning_nums)
    }
}


#[cfg(test)]
mod tests {
    use crate::process;
    use crate::extract_index;
    use crate::extract_my_nums;
    #[test]
    fn basic_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(extract_index(input), 1);
        assert_eq!(extract_my_nums(input), vec![41, 48, 83, 86, 17]);
        assert_eq!(process(input.to_string()), 13);
    }
}
