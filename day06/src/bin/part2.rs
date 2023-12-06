use std::{fs, iter::zip};
fn main() {
    let contents = fs::read_to_string("src/part2-input.txt").expect("Could not read file.");

//     let contents = "Time:      71530
// Distance:  940200".to_string();

    let result = process(contents);
    println!("{}", result);

}

fn process(contents: String) -> usize {

    let races = construct(contents);

    races[0].number_of_ways_to_beat()
}

fn construct(contents: String) -> Vec<Race> {
    let times: Vec<usize> = contents.split("\n")
            .collect::<Vec<&str>>()[0]
            .split(" ")
            .filter_map(|i| i.parse().ok())
            .collect();
            
    let distances: Vec<usize> = contents.split("\n")
            .collect::<Vec<&str>>()[1]
            .split(" ")
            .filter_map(|i| i.parse().ok())
            .collect();

    let mut races : Vec<Race> = Vec::new();
    for (time, distance) in zip(times, distances) {
        races.push(Race {
            time, distance
        });
    }
    races
}

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize
}

impl Race {
    fn can_win(&self, num: usize) -> bool {
        (self.time - num) * num > self.distance
    }

    fn number_of_ways_to_beat(&self) -> usize {
        let mut low: usize = 0;
        let mut high: usize = (self.time / 2) - 1;
        let mut mid: usize = 0;

        while low <= high {
            mid = ((high - low) / 2) + low;
            
            if self.can_win(mid) {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }

        self.time - (mid * 2) + 1

    }
}

