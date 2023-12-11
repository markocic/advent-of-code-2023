use std::{fs, collections::HashSet};

type Point = (usize, usize);

const NORTH: (isize, isize) = (-1, 0);
const SOUTH: (isize, isize) = (1, 0);
const WEST: (isize, isize) = (0, -1);
const EAST: (isize, isize) = (0, 1);

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("Could not read from file.");

    let result = process(contents);
    println!("{}", result);
}

fn process(contents: String) -> usize {
    let matrix = parse_input(contents);

    let start = find_s(&matrix);

    find_chain(&matrix, start)
}

fn find_chain(matrix: &Vec<Vec<char>>, start: Point) -> usize {

    let mut visited: HashSet<Point> = HashSet::new();
    let mut stack : Vec<Point> = Vec::new();

    stack.push(start);

    while !stack.is_empty() {

        let curr = stack.pop().unwrap();
        visited.insert(curr);

        for direction in find_valid_directions(matrix[curr.0][curr.1]) {
            let n : (isize, isize) = (curr.0 as isize + direction.0, curr.1 as isize + direction.1);
            println!("is {:?} a valid point? ", n);

            if !is_valid(&matrix, n) { continue; }
            println!("YES!");

            let n : Point = (n.0 as usize, n.1 as usize);

            if visited.contains(&n) { continue; }

            println!("Checking point {:?}", n);

            if is_neighbor(matrix, &n, direction) {
                println!("Point {:?} is a valid neighbor", n);
                stack.push(n);
            }
        }
    }
    println!("Path taken: {:?}", visited);
    visited.len() / 2
}

fn find_valid_directions(c: char) -> Vec<(isize, isize)> {

    match c {
        '|' => vec![NORTH, SOUTH],
        'S' => vec![NORTH, SOUTH, WEST, EAST],
        '7' => vec![SOUTH, WEST],
        'F' => vec![SOUTH, EAST],
        'J' => vec![NORTH, WEST],
        'L' => vec![NORTH, EAST],
        '-' => vec![EAST, WEST],

        _ => vec![]
    }    
}

fn is_valid(matrix: &Vec<Vec<char>>, n: (isize, isize)) -> bool {
    n.0 >= 0 && n.0 < matrix.len() as isize && n.1 >= 0 && n.1 < matrix[0].len() as isize
}

fn is_neighbor(matrix: &Vec<Vec<char>>, n: &Point, dir: (isize, isize)) -> bool {
    println!("Got point {:?}, found char {:?}, and dir ({:?})", n, matrix[n.0][n.1], dir);
    match dir {
        NORTH => {
            matrix[n.0][n.1] == '7' || matrix[n.0][n.1] == '|' || matrix[n.0][n.1] == 'F'
        },
        SOUTH => {
            matrix[n.0][n.1] == 'L' || matrix[n.0][n.1] == '|' || matrix[n.0][n.1] == 'J'
        },
        WEST => {
            matrix[n.0][n.1] == '-' || matrix[n.0][n.1] == 'L' || matrix[n.0][n.1] == 'F'
        },
        EAST => {
            matrix[n.0][n.1] == '-' || matrix[n.0][n.1] == '7' || matrix[n.0][n.1] == 'J'
        },
        _ => unreachable!("should not happen")
    }
}

fn find_s(matrix: &Vec<Vec<char>>) -> Point {
    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 's' || *c == 'S' { return (i, j); }
        }
    }

    unreachable!("Should not happen");
}

fn parse_input(contents: String) -> Vec<Vec<char>> {
    contents.split("\n")
        .map(|s| s.chars().collect::<Vec<char>>())
        .filter(|s| !s.is_empty())
        .collect()
}

#[cfg(test)]
mod test {
    use crate::process;

    #[test]
    fn test() {
        let t = ".....
.S-7.
.|.|.
.L-J.
....."
            .to_string();

        assert_eq!(process(t), 4);
    }

    #[test]
    fn test1() {
        let t = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF"
            .to_string();

        assert_eq!(process(t), 4);
    }

    #[test]
    fn test2() {
        let t = "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
            .to_string();

        assert_eq!(process(t), 8);
    }

    #[test]
    fn test3() {
        let t = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"
            .to_string();

        assert_eq!(process(t), 8);
    }
}
