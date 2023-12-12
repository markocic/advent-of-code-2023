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
        result.push(Springs {
            record: split[0].to_string(),
            damaged: split[1]
                .split(",")
                .filter_map(|i| i.parse().ok())
                .collect()
        })
    }

    result
}

#[derive(Debug)]
struct Springs {
    record: String,
    damaged: Vec<usize>
}

impl Springs {

    fn number_of_combinations(&self) -> usize {
        0
    }

    fn compute(&self, length: usize) -> usize {

        0
    }
}

#[cfg(test)]
mod test {
    use crate::process;

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
}
