use std::fs;

const RED_COUNT: isize = 12;
const GREEN_COUNT: isize = 13;
const BLUE_COUNT: isize = 14;


fn main() {

    let contents = fs::read_to_string("src/input.txt")
        .expect("Error reading file.");
    let lines = contents.split("\n");

    let mut sum: isize = 0;
    let mut current_game_index: isize = 1;
    let mut cubes: (isize, isize, isize) = (RED_COUNT, GREEN_COUNT, BLUE_COUNT); // R G B
    for line in lines {
        let game: Vec<&str> = line.split(":").collect();

        if game.len() < 2 {continue;}
        let rounds = game[1].split(";");
        println!("current line: {}", line);

        for round in rounds {
            let instances = round.split(",");

            for instance in instances {
                if instance.contains("red") {
                    cubes.0 -= get_int(instance);
                } else if instance.contains("green") {
                    cubes.1 -= get_int(instance);
                } else if instance.contains("blue") {
                    cubes.2 -= get_int(instance);
                }

                println!("({} {} {}), get_int = {}", cubes.0, cubes.1, cubes.2, get_int(instance));
            }

            if !check_remaining(cubes) {
                println!("failed");
                break;
            }

            reset_cubes(&mut cubes);
        }

        if check_remaining(cubes) {
            sum += current_game_index;
        }

        current_game_index += 1;
        reset_cubes(&mut cubes);

    }

    println!("{}", sum);

}

fn check_remaining(cubes: (isize, isize, isize)) -> bool {
    return cubes.0 >= 0 && cubes.1 >= 0 && cubes.2 >= 0

}

fn reset_cubes(cubes: &mut(isize, isize, isize)) {
    *cubes = (RED_COUNT, GREEN_COUNT, BLUE_COUNT);
}

fn get_int(instance: &str) -> isize {
    let num:Vec<&str> = instance.trim().split(" ").collect();
    println!(":{}: {}", instance, num[0]);
    return match num[0].parse() {
        Ok(res) => res,
        Err(_) => {0},
    };
}
