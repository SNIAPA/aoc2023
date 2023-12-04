use std::collections::HashMap;

#[cfg(test)]
pub mod test;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Color {
    Red,
    Green,
    Blue,
}

pub fn sum_possible_games(input: &str) -> usize {
    let games = parse_input(input);


    games.iter().fold(0, |acc, (_, game)| {
        let mut min_limits = HashMap::<Color, usize>::from([(Color::Red, 0usize),(Color::Green, 0usize),(Color::Blue, 0usize)]);

        for round in game {
            for (count,color) in round {
                let limit = min_limits.get_mut( color).unwrap();
                if *limit < *count {
                    *limit = *count
                }

            }
        }
        
        acc + min_limits.into_values().reduce(|acc,x| acc * x).unwrap()
    })
}

pub fn parse_input(input: &str) -> Vec<(usize,Vec<Vec<(usize,Color)>>)> {
    input.split("\n").map(|line| {
        let mut split_line = line.split(": ");
        let title = split_line.next().unwrap();
        let content = split_line.next().unwrap();

        let id = usize::from_str_radix(title.split(" ").nth(1).unwrap(), 10).unwrap();
        let subsets = content
            .split("; ")
            .map(|colors| {
                let split_colors = colors.split(", ");
                split_colors
                    .map(|colors| {
                        let mut split_color = colors.split(" ");
                        let count = usize::from_str_radix(split_color.next().unwrap(), 10).unwrap();
                        let color = match split_color.next().unwrap() {
                            "red" => Color::Red,
                            "green" => Color::Green,
                            "blue" => Color::Blue,
                            _ => todo!()
                        };
                        (count, color)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        (id,subsets)

    }).collect::<Vec<_>>()

}

