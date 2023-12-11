use std::fs;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let contents = fs::read_to_string("src/input.txt").expect("Could not read from file.");

    let result = process(contents);
    println!("{}", result);
}

type Point = (usize, usize);

fn process(contents: String) -> usize {
    let matrix = parse_input(contents);
    print_matrix(matrix.clone());

    let empty_rows = find_empty_rows(&matrix);
    let empty_cols = find_empty_cols(&matrix);

    println!("{:?} {:?}", empty_rows, empty_cols);

    let galaxies = find_galaxies(&matrix);

    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            let dist = find_distance(galaxies[i], galaxies[j], &empty_rows, &empty_cols);
            // println!("distance between {:?} and {:?} is {}", galaxies[i], galaxies[j], dist);
            sum += dist;
        }
    }

    print_matrix(matrix.clone());
    sum
}

fn find_distance(first: Point, second: Point, empty_rows: &Vec<usize>, empty_cols: &Vec<usize>) -> usize {
    let max_x = first.0.max(second.0);
    let min_x = first.0.min(second.0);
    
    let max_y = first.1.max(second.1);
    let min_y = first.1.min(second.1);

    let row_count = empty_rows.iter().filter(|&i| max_x > *i && min_x < *i).count();
    let col_count = empty_cols.iter().filter(|&i| max_y > *i && min_y < *i).count();

    (max_x - min_x) + (max_y - min_y) + (row_count + col_count)

}

fn find_galaxies(matrix: &Vec<Vec<char>>) -> Vec<Point> {
    let mut result : Vec<Point> = Vec::new();

    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '#' { result.push((i, j)) }
        }
    }

    result
}

fn find_empty_rows(matrix: &Vec<Vec<char>>) -> Vec<usize> {
    let mut flag;
    let mut result = Vec::new();
    for (i, row) in matrix.iter().enumerate() {
        flag = true;

        for c in row {
            if *c != '.' { flag = false; }
        }
        if flag { result.push(i)}
    }

    result
}

fn find_empty_cols(matrix: &Vec<Vec<char>>) -> Vec<usize> {

    let mut result : Vec<usize> = (0..matrix[0].len()).collect();
    println!("{:?}", result);

    for row in matrix {
        for (i, c) in row.iter().enumerate() {
            if *c != '.' { result.retain(|j| *j != i) }
        }
    }

    result
}

fn parse_input(contents: String) -> Vec<Vec<char>> {
    contents.split("\n")
        .map(|s| s.split("").filter_map(|i| i.parse().ok())
         .collect::<Vec<char>>()).collect()
}

fn print_matrix(matrix: Vec<Vec<char>>) {
    for row in matrix {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use crate::{process, parse_input};

    #[test]
    fn test() {
        let t = "\
...#.......
.......#...
#..........
...........
......#....
.#.........
.........#.
...........
.......#...
#...#......".to_string();
        
        println!("{:?}", parse_input(t.clone()));
        
        assert_eq!(process(t), 374);
    }
}
