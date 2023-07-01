// use std::io;
use crate::challenges::sorting;
use std::{fs, num::IntErrorKind};

const FILE_INPUT: &str = "src/challenges/day_01/input.txt";

pub fn calculate_max_calories() {
    let data = fs::read_to_string(FILE_INPUT).unwrap();
    let lines = data.lines().map(|line| line.parse::<u32>());
    let mut elves_count: usize = 0;
    let mut calories_held: Vec<u32> = vec![0];

    for line in lines {
        // With match
        match line {
            Ok(data) => {
                calories_held[elves_count] = calories_held[elves_count] + data;
            }
            Err(error) => {
                if IntErrorKind::Empty.eq(error.kind()) {
                    elves_count += 1;
                    calories_held.push(0);
                } else {
                    panic!("Error ocurred {:?}", error);
                }
            }
        }

        // With unwrap
        // let data = line.unwrap_or_else(|error| {
        //     if IntErrorKind::Empty.eq(error.kind()) {
        //         elves_count += 1;
        //         calories_held.push(0);
        //         return 0;
        //     } else {
        //         panic!("Error ocurred {:?}", error);
        //     }
        // });

        // calories_held[elves_count] = calories_held[elves_count] + data;

        // I prefer match in this case, because in case of IntErrorKind::Empty I don't want to return any value
        // but to make a process to continue the flow. That way I can get rid of unnecessary return 0 in line 19
        // and a comparation.
    }
    sorting::insertion_sort(&mut calories_held);
    print!("{:?}", calories_held.last().unwrap())
}
