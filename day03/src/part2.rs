use std::fs;

fn main() {
    let filepath = "src/edge_cases.txt";

    let contents = fs::read_to_string(filepath).expect("Error reading the file.");

    let lines: Vec<&str> = contents.split("\n").collect();

    let count_matrix = generate_count_matrix(&lines);
    let product_matrix = generate_product_matrix(&lines);

    let mut i = 0;
    let mut j = 0;
    let mut sum = 0;
    const ADJACENT: usize = 2;
    for row in &count_matrix {
        for count in row {
            if *count == ADJACENT {
                sum += product_matrix.get(i).unwrap().get(j).unwrap();
            }

            j += 1;
        }
        j = 0;
        i += 1;
    }

    display_matrix(count_matrix);
    display_matrix(product_matrix);
    println!("{}", sum);
}

fn generate_product_matrix(lines: &Vec<&str>) -> Vec<Vec<usize>> {
    let mut result = generate_empty_vec(1, lines.len(), lines.get(0).unwrap().len());

    let mut i = 0;
    for line in lines {
        if line.len() < 1 {
            continue;
        }

        let mut num = String::from("");

        for (j, c) in line.chars().enumerate() {
            if is_numeric_char(c) {
                num += &c.to_string();
            } else {
                if !num.is_empty() {
                    set_product(&mut result, num.parse::<usize>().unwrap(), i, j, num.len());
                }
                num = String::from("");
            }
        }

        if !num.is_empty() {
            set_product(
                &mut result,
                num.parse::<usize>().unwrap(),
                i,
                lines.get(0).unwrap().len() - 1,
                num.len(),
            );
        }
        i += 1;
    }

    return result;
}

fn set_product(product: &mut Vec<Vec<usize>>, num: usize, x: usize, y: usize, length: usize) {
    let x: isize = x.try_into().expect("Could not convert");
    let y: isize = y.try_into().expect("Could not convert");
    let length: isize = length.try_into().expect("Could not convert");

    for i in (x - 1)..(x + 2) {
        for j in (y - length - 1)..(y + 1) {
            if i < 0 || j < 0 {
                continue;
            }
            let i: usize = i.try_into().expect("Could not convert");
            let j: usize = j.try_into().expect("Could not convert");

            if i >= product.len() || j >= product.get(0).unwrap().len() {
                continue;
            }
            product[i][j] *= num;
        }
    }
}

fn generate_empty_vec(value: usize, x: usize, y: usize) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = Vec::new();

    for _i in 0..x {
        let mut row: Vec<usize> = Vec::new();
        for _j in 0..y {
            row.push(value);
        }
        result.push(row);
    }

    return result;
}

fn generate_count_matrix(lines: &Vec<&str>) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = Vec::new();

    let mut i = 0;
    for line in lines {
        let mut row: Vec<usize> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            if c == '*' {
                row.push(find_count(&lines, i, j));
            } else {
                row.push(0);
            }
        }
        result.push(row);
        i += 1;
    }

    return result;
}

fn find_count(lines: &Vec<&str>, x: isize, y: usize) -> usize {
    let y: isize = y.try_into().expect("Error converting");

    let mut last_was_numeric: bool = false;
    let mut count = 0;
    for i in (x - 1)..(x + 2) {
        for j in (y - 1)..(y + 2) {
            if is_numeric(&lines, i, j) {
                if last_was_numeric {
                    continue;
                }
                count += 1;
                last_was_numeric = true;
            } else {
                last_was_numeric = false;
            }
        }
    }

    return count;
}

fn is_numeric(contents: &Vec<&str>, x: isize, y: isize) -> bool {
    match (usize::try_from(x), usize::try_from(y)) {
        (Ok(x), Ok(y)) => {
            let c = contents
                .get(x)
                .and_then(|s| s.chars().nth(y))
                .filter(|&c| is_numeric_char(c));
            c.is_some()
        }
        _ => false,
    }
}

fn is_numeric_char(c: char) -> bool {
    return c >= '0' && c <= '9';
}

fn display_matrix(matrix: Vec<Vec<usize>>) {
    for row in matrix {
        for c in row {
            print!("{} ", c);
        }
        println!();
    }
}
