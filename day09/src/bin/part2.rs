use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("Could not read from file.");

    let result = process(contents);
    println!("{}", result);
}

fn process(contents: String) -> i32 {
    let seqs = parse_input(contents);
    println!("{:?}", seqs);

    let mut sum = 0;
    for mut seq in seqs {
        if seq.is_empty() { continue; }
        seq.reverse();
        let tmp = *seq.last().unwrap() + find_next(seq);
        sum += tmp;
    }

    sum
}

fn find_next(seq: Vec<i32>) -> i32 {
    if seq.len() == 1 { return seq[0]; }

    let mut not_zeros = false;
    let new_seq :Vec<i32> = seq.windows(2)
        .into_iter()
        .map(|x| {
            x[1] - x[0]
        })
        .collect();

    new_seq.iter().for_each(|i| if *i != 0_i32 { not_zeros = true; });
    
    if not_zeros {
        return *new_seq.last().unwrap() + find_next(new_seq);
    } else {
        return 0
    }
}

fn parse_input(contents: String) -> Vec<Vec<i32>> {
    contents.split("\n")
        .map(|s| s.split(" ")
             .filter_map(|i| i.parse().ok())
             .collect())
         .collect()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_case() {
        let t = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45".to_string();

        assert_eq!(process(t), 114);
    }

}
