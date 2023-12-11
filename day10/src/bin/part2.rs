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

    let main_loop = find_chain(&matrix, start);

    let denoisified_matrix = denoisify(matrix, main_loop);

    count_enclosed(denoisified_matrix)
}

fn count_enclosed(matrix: Vec<Vec<char>>) -> usize {
    let mut count = 0;
    let mut flag;

    println!("___________________________");
    for row in matrix
             .iter()
             .map(|i| i.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(""))
             .collect::<Vec<String>>() {
                 println!("{}", row);
             }
    println!("___________________________");

    for row in matrix {
        flag = false;
        let mut prev_corner : char = '0';
        for c in row {

            match c {
                '|' | 'S' => flag = !flag,
                'F' | 'L' => {
                    flag = !flag;
                    prev_corner = c;
                }
                '7' => {
                    if prev_corner == 'F' { flag = !flag }
                }
                'J' => {
                    if prev_corner == 'L' { flag = !flag }
                }
                '.' => if flag { count += 1 },
                _ => {}
            }
            print!("{f}", f = if flag { if c == '.' { 2 } else { 1 } } else { 0 });
        }
        println!();
    }

    count

}

fn denoisify(matrix: Vec<Vec<char>>, main_loop: HashSet<Point>) -> Vec<Vec<char>> {
    let mut result : Vec<Vec<char>> = Vec::new();

    for i in 0..matrix.len() {
        let mut row : Vec<char> = Vec::new();
        for j in 0..matrix[1].len() {
            if main_loop.contains(&(i, j)) {
                row.push(matrix[i][j]);
            } else { row.push('.'); }
        }
        result.push(row);
    }

    let start = find_s(&result);
    let directions = get_node_directions(&result, start);

    result[start.0][start.1] = convert_direction(directions);

    result
}

fn convert_direction(directions: Vec<(isize, isize)>) -> char {
    let mut possible = vec!['|', 'F', '7', 'J', 'L', '-'];

    for direction in directions {
        match direction {
            NORTH => possible.retain(|d| *d != '|' || *d != 'J' || *d != 'L'),
            SOUTH => possible.retain(|d| *d != '|' || *d != 'F' || *d != '7'),
            EAST => possible.retain(|d| *d != 'F' || *d != 'L' || *d != '-'),
            WEST => possible.retain(|d| *d != '7' || *d != 'J' || *d != '-'),
            _ => unreachable!("Should not happen")
        }
    }

    possible[0]
}

fn get_node_directions(matrix: &Vec<Vec<char>>, start: Point) -> Vec<(isize, isize)> {

    let mut result : Vec<(isize, isize)> = Vec::new();

    for direction in find_valid_directions(matrix[start.0][start.1]) {
        let n : (isize, isize) = (start.0 as isize + direction.0, start.1 as isize + direction.1);
        if !is_valid(&matrix, n) { continue; }
        let n : Point = (n.0 as usize, n.1 as usize);


        if is_neighbor(&matrix, &n, direction) {
            result.push(direction);
        }
    }

    result
}

fn find_chain(matrix: &Vec<Vec<char>>, start: Point) -> HashSet<Point> {

    let mut visited: HashSet<Point> = HashSet::new();
    let mut stack : Vec<Point> = Vec::new();

    stack.push(start);

    let mut directions : Vec<(isize, isize)> = Vec::new();

    while !stack.is_empty() {

        let curr = stack.pop().unwrap();
        visited.insert(curr);

        for direction in find_valid_directions(matrix[curr.0][curr.1]) {
            let n : (isize, isize) = (curr.0 as isize + direction.0, curr.1 as isize + direction.1);
            if !is_valid(&matrix, n) { continue; }
            let n : Point = (n.0 as usize, n.1 as usize);

            if visited.contains(&n) { continue; }

            if is_neighbor(matrix, &n, direction) {
                stack.push(n);

                if directions.len() < 2_usize {
                    directions.push(direction);
                }
            }
        }
    }
    visited
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
    use crate::{process, denoisify};

    #[test]
    fn test() {
        let t = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
            .to_string();

        assert_eq!(process(t), 8);
    }

    #[test]
    fn test1() {
        let t = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
            .to_string();

        assert_eq!(process(t), 10);
    }

    #[ignore]
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

    #[ignore]
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

    use crate::{parse_input, find_s, find_chain};

    #[ignore]
    #[test]
    fn test4() {
        let t = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L".to_string();
//         let t = "7-F7-
// .FJ|7
// SJLL7
// |F--J
// LJ.LJ".to_string();

        let matrix = parse_input(t);

        let start = find_s(&matrix);

        let visited = find_chain(&matrix, start);
        for row in denoisify(matrix, visited)
                 .iter()
                 .map(|i| i.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(""))
                 .collect::<Vec<String>>() {
                     println!("{}", row);
                 }

        assert_eq!(0, 1);

    }
}
