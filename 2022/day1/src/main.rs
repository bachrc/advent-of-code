use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Elfo {
    pub calories: i32
}

fn main() {
    let file = File::open("day1/input.txt").expect("file not found!");
    let reader = BufReader::new(file);

    let mut current_elve_calories = 0;

    let mut elfos_list: Vec<Elfo> = Vec::new();

    for parsed_line in reader.lines() {
        let line_content: String = parsed_line.expect("Invalid line content");
        if line_content.is_empty() {
            elfos_list.push(Elfo{
                calories: current_elve_calories
            });
            current_elve_calories = 0;
        } else {
            let current_calorie_count: i32 = line_content.parse().expect("It is not a number this line.");
            current_elve_calories += current_calorie_count
        }
    }

    elfos_list.sort_by(|a: &Elfo, b: &Elfo| {
        if a.calories > b.calories {
            Ordering::Less
        } else if a.calories == b.calories {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });

    println!("The maximum elve is {}", elfos_list[0].calories);
    let total_of_first_three = elfos_list[0].calories + elfos_list[1].calories + elfos_list[2].calories;
    println!("The three best elfos have in total {} calories ", total_of_first_three)
}
