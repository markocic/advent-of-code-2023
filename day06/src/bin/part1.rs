use std::{fs, iter::zip};
fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("Could not read file.");

//     let contents = "Time:      7  15   30
// Distance:  9  40  200".to_string();

    let result = process(contents);
    println!("{}", result);

}

fn process(contents: String) -> usize {

    let races = construct(contents);
    races.iter().map(|r| r.number_of_ways_to_beat()).product()
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
        let mut counter = 0;
        for i in 0..self.time {
            if self.can_win(i) {
                counter += 1;
            }
        }

        counter

    }
}

