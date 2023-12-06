use std::fs;
use std::str;

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("Could not open file.");

//     let contents = "seeds: 79 14 55 13
//
// seed-to-soil map:
// 50 98 2
// 52 50 48
//
// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15
//
// fertilizer-to-water map:
// 49 53 8
// 0 11 42
// 42 0 7
// 57 7 4
//
// water-to-light map:
// 88 18 7
// 18 25 70
//
// light-to-temperature map:
// 45 77 23
// 81 45 19
// 68 64 13
//
// temperature-to-humidity map:
// 0 69 1
// 1 0 69
//
// humidity-to-location map:
// 60 56 37
// 56 93 4
// ";

    let result = process(contents.to_string());
    println!("{}", result);
}

fn process(contents: String) -> usize {
    let lines = contents.split("\n");
    let mut seeds : Vec<usize> = Vec::new();
    let mut from: Type = Type::SEED;
    let mut to: Type = Type::SOIL;
    let mut rules: Vec<Rule> = Vec::new();
    let mut conversion_map: ConversionMap = ConversionMap { conversions: Vec::new() };

    for line in lines {
        if line.contains("seeds: ") {
            seeds = parse_seeds(line);
            continue;
        }

        if line.contains("-to-") {
            (from, to) = parse_type(line);
            continue;
        }

        if line.is_empty() {
            let conversion = Conversion {
                from: from.clone(),
                to: to.clone(),
                rules: rules.clone()
            };
            conversion_map.conversions.push(conversion);
            rules = Vec::new();
            continue;
        }

        rules.push(parse_rule(line));
    }

    let mut min: usize = usize::MAX;

    for seed in seeds {
        min = min.min(conversion_map.convert(seed));
    }
    min
}

fn parse_rule(line: &str) -> Rule {
    let nums :Vec<usize> =
        line.split(" ")
        .filter_map(|i| i.parse().ok())
        .collect();

    Rule { 
        destination_start: nums[0],
        source_start: nums[1],
        length: nums[2]
    }

}

fn parse_type(line: &str) -> (Type, Type) {
    let from: Type = match line.split("-to-").collect::<Vec<&str>>()[0].parse() {
        Ok(x) => x,
        Err(_) => panic!("Could not parse from conversion for line: \"{}\"", line),
    };

    let to: Type = match line.split("-to-").collect::<Vec<&str>>()[1]
        .split(" ")
        .collect::<Vec<&str>>()[0]
        .parse()
    {
        Ok(x) => x,
        Err(_) => panic!("Could not parse to conversion for line: \"{}\"", line),
    };

    (from, to)
}

fn parse_seeds(line: &str) -> Vec<usize> {
    line.split(" ")
        .filter_map(|i| i.parse::<usize>().ok())
        .collect::<Vec<usize>>()
}

struct ConversionMap {
    conversions: Vec<Conversion>,
}

impl ConversionMap {
    fn convert(&self, num: usize) -> usize {
        let mut current = num;
        for conversion in &self.conversions {
            current = conversion.convert(current);
        }
        current
    }
}

struct Conversion {
    from: Type,
    to: Type,
    rules: Vec<Rule>
}

impl Conversion {
    fn convert(&self, num: usize) -> usize {
        for rule in &self.rules {
            match rule.convert(num) {
                Some(x) => return x,
                None => continue,
            };
        }
        num
    }
}

#[derive(Clone)]
struct Rule {
    source_start: usize,
    destination_start: usize,
    length: usize,
}

impl Rule {
    fn convert(&self, num: usize) -> Option<usize> {
        if self.in_range(num) {
            return Some(num + self.destination_start - self.source_start);
        } else {
            return None;
        }
    }

    fn in_range(&self, num: usize) -> bool {
        return num >= self.source_start && num < self.source_start + self.length;
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Type {
    SEED,
    SOIL,
    FERTILIZER,
    WATER,
    LIGHT,
    TEMPERATURE,
    HUMIDITY,
    LOCATION,
}

impl str::FromStr for Type {
    type Err = ();

    fn from_str(input: &str) -> Result<Type, Self::Err> {
        match input {
            "seed" => Ok(Type::SEED),
            "soil" => Ok(Type::SOIL),
            "fertilizer" => Ok(Type::FERTILIZER),
            "water" => Ok(Type::WATER),
            "light" => Ok(Type::LIGHT),
            "temperature" => Ok(Type::TEMPERATURE),
            "humidity" => Ok(Type::HUMIDITY),
            "location" => Ok(Type::LOCATION),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Conversion, Rule};

    #[test]
    fn rule_conversion() {
        let rule = Rule {
            source_start: 98,
            destination_start: 50,
            length: 2,
        };
        assert_eq!(rule.convert(98_usize).unwrap(), 50);
        assert_eq!(rule.convert(99_usize).unwrap(), 51);
        assert_eq!(rule.convert(100_usize), None);
    }

    #[test]
    fn conversion() {
        let rule1 = Rule {
            source_start: 98,
            destination_start: 50,
            length: 2,
        };
        let rule2 = Rule {
            source_start: 50,
            destination_start: 52,
            length: 48,
        };

        let conversion = Conversion {
            from: crate::Type::SEED,
            to: crate::Type::SOIL,
            rules: vec![rule1, rule2],
        };

        assert_eq!(conversion.convert(97), 99);
        assert_eq!(conversion.convert(98), 50);
        assert_eq!(conversion.convert(50), 52);
        assert_eq!(conversion.convert(99), 51);
        assert_eq!(conversion.convert(100), 100);
        assert_eq!(conversion.convert(0), 0);
        assert_eq!(conversion.convert(0), 0);
    }
}
