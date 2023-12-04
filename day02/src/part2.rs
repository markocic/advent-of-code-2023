use std::fs;

fn main() {
    
    let contents = fs::read_to_string("src/input.txt")
        .expect("Error reading file.");
    let lines = contents.split("\n");


    let mut sum: isize = 0;
    let mut minimal_cubes: (isize, isize, isize) = (-1, -1, -1);
    let mut current_cubes: (isize, isize, isize) = (0, 0, 0);

    for line in lines {
        let game: Vec<&str> = line.split(":").collect();

        if game.len() < 2 { continue; }
        let rounds = game[1].split(";");

        for round in rounds {
            let instances = round.split(",");

            for instance in instances {

                if instance.contains("red") {
                    current_cubes.0 += get_int(instance);
                } else if instance.contains("green") {
                    current_cubes.1 += get_int(instance);
                } else if instance.contains("blue") {
                    current_cubes.2 += get_int(instance);
                }

            }
            update_minimal_cubes(&mut minimal_cubes, current_cubes);
            reset_cubes(&mut current_cubes);
        }

        sum += find_product(minimal_cubes);
        reset_cubes(&mut minimal_cubes);
    }

    println!("{}", sum);

}

fn reset_cubes(cubes: &mut(isize, isize, isize)) {
    *cubes = (0, 0, 0);
}

fn update_minimal_cubes(minimal: &mut(isize, isize, isize), current: (isize, isize, isize)) {
    minimal.0 = std::cmp::max(minimal.0, current.0);
    minimal.1 = std::cmp::max(minimal.1, current.1);
    minimal.2 = std::cmp::max(minimal.2, current.2);
}

fn find_product(cubes: (isize, isize, isize)) -> isize {
    cubes.0 * cubes.1 * cubes.2
}

fn get_int(instance: &str) -> isize {
    let num:Vec<&str> = instance.trim().split(" ").collect();
    return match num[0].parse() {
        Ok(res) => res,
        Err(_) => {0},
    };
}
