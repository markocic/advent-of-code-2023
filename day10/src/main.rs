use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("Could not read from file.");

    let result = process(contents);
    println!("{}", result);
}

fn process(contents: String) -> usize {
    0
}

#[cfg(test)]
mod test {
    use crate::process;

    #[test]
    fn test() {
        let t = ".....
.s-7.
.|.|.
.l-j.
....."
            .to_string();

        assert_eq!(process(t), 4);
    }

    #[test]
    fn test1() {
        let t = "-l|f7
7s-7|
l|7||
-l-j|
l|-jf"
            .to_string();

        assert_eq!(process(t), 4);
    }

    #[test]
    fn test2() {
        let t = "..f7.
.fj|.
sj.l7
|f--j
lj..."
            .to_string();

        assert_eq!(process(t), 8);
    }

    #[test]
    fn test3() {
        let t = "7-f7-
.fj|7
sjll7
|f--j
lj.lj"
            .to_string();

        assert_eq!(process(t), 8);
    }
}
