use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("Could not read file.");

    let result = process(contents);
    println!("{}", result);
}

fn process(contents: String) -> usize {
    let moves = parse_moves(&contents);
    let graph = generate_graph_nodes(&contents);

    find_zzz(graph, moves)
}

fn find_zzz(graph: HashMap<String, Node>, moves: Vec<Move>) -> usize {
    
    let mut head = graph.get("AAA").unwrap();
    let mut counter = 0;

    for mov in moves.iter().cycle() {
        if head.text == "ZZZ".to_owned() { break; }
        counter += 1;

        head = match mov {
            Move::Left => graph.get(&head.left).unwrap(),
            Move::Right => graph.get(&head.right).unwrap(),
        }
    }

    counter
}


fn generate_graph_nodes(contents: &String) -> HashMap<String, Node> {
    let mut map : HashMap<String, Node> = HashMap::new();
    let mut lines : Vec<&str> = contents.split("\n").collect();
    let _ = &lines.remove(0);

    for line in lines {
        if line.is_empty() { continue; }
        let main = &line[0..3];

        let left = &line[7..10];
        let right = &line[12..15];

        map.insert(main.to_string(), Node {
            text: main.to_string(),
            left: left.to_string(),
            right: right.to_string()
        });
    }
    map
}

fn parse_moves(contents: &String) -> Vec<Move> {
    contents.split("\n")
        .collect::<Vec<&str>>()[0]
        .chars()
        .into_iter()
        .map(|c| to_move(c))
        .collect()
}

fn to_move(c: char) -> Move {
    if c == 'R' { return Move::Right }
    else if c == 'L' { return Move::Left }
    panic!("Could not parse move!");
}

#[derive(Debug)]
enum Move {
    Left, Right
}

#[derive(Debug, Clone)]
struct Node {
    text: String,
    left: String,
    right: String
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_cases() {

        let contents1 = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)".to_string();

    let contents2 = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)".to_string();

        assert_eq!(process(contents1), 2);
        assert_eq!(process(contents2), 6);
    }
}
