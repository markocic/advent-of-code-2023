use std::fs;
use std::string::String;

fn main() {
    let filepath = "src/input.txt";

    let contents = fs::read_to_string(filepath).expect("Error reading the file.");

    let lines = to_matrix(contents);
    let mut i = 0;
    let mut j = 0;
    let mut sum = 0;
    for line in &lines {
        if line.len() < 1 {
            continue;
        }

        let mut num = String::from("");
        for c in line.chars() {
            if is_numeric(c) {
                num += &c.to_string();
            } else {
                if !num.is_empty() {
                    // println!("Found num: {}", num);
                    if is_part_number(
                        &lines,
                        i,
                        j,
                        num.len().try_into().expect("Could not convert"),
                    ) {
                        sum += num.parse::<isize>().unwrap();
                        // println!("Num is part number: {}", num);
                    }
                }
                num = String::from("");
            }

            j += 1;
        }

        if !num.is_empty() {
            // println!("Found num: {}", num);
            if is_part_number(
                &lines,
                i,
                j,
                num.len().try_into().expect("Could not convert"),
            ) {
                sum += num.parse::<isize>().unwrap();
                // println!("Num is part number: {}", num);
            }
        }
        j = 0;
        i += 1;
    }

    println!("{}", sum);
}

fn is_part_number(contents: &Vec<String>, x: isize, y: isize, length: isize) -> bool {
    // println!("checking coords: {} {} with len {}", x, y, length);

    for i in (x - 1)..(x + 2) {
        // println!("{}", i);
        for j in (y - length - 1)..(y + 1) {
            // println!("{}", j);
            if is_symbol(&contents, i, j) {
                return true;
            }
        }
    }
    return false;
}

fn is_symbol(contents: &[String], x: isize, y: isize) -> bool {
    match (usize::try_from(x), usize::try_from(y)) {
        (Ok(x), Ok(y)) => {
            let c = contents
                .get(x)
                .and_then(|s| s.chars().nth(y))
                .filter(|&c| (c != '.' && !is_numeric(c)));
            c.is_some()
        }
        _ => false,
    }
}

fn to_matrix(contents: String) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    for line in contents.split("\n") {
        result.push(line.to_string());
    }

    result
}

fn is_numeric(c: char) -> bool {
    return c >= '0' && c <= '9';
}
