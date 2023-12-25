use std::fmt::Debug;

struct Draw {
    blue: u8,
    green: u8,
    red: u8,
}

impl Debug for Draw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} red, ", self.red)?;
        write!(f, "{} blue, ", self.blue)?;
        write!(f, "{} green, ", self.green)?;
        Ok(())
    }
}

fn parse_game(game: &str) -> (u16, Vec<Draw>) {
    let (game_id, s) = game.split_once(':')
        .map(|(id, draws)| (
            id.trim().split_ascii_whitespace().nth(1).unwrap().parse::<u16>().unwrap(),
            draws.split(';')
                .map(|draw| draw.split(',')
                    .map(|s| s.trim().split_once(' ')
                        .map(|(n, color)|
                            match color {
                                "red" => Draw { blue: 0, green: 0, red: n.parse::<u8>().unwrap() },
                                "blue" => Draw { blue: n.parse::<u8>().unwrap(), green: 0, red: 0 },
                                "green" => Draw { blue: 0, green: n.parse::<u8>().unwrap(), red: 0 },
                                _ => panic!("Unknown color: {color}"),
                            }).unwrap())
                    .fold(Draw { blue: 0, green: 0, red: 0 }, |acc, draw| Draw {
                        blue: acc.blue + draw.blue,
                        green: acc.green + draw.green,
                        red: acc.red + draw.red,
                    }))
                .collect::<Vec<_>>()
        )).unwrap();
    (game_id, s)
}

fn main() {
    let game = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
    let (game_id, s) = parse_game(game);

    assert_eq!(game_id, 3);
    println!("{:?}", game_id, );
    s.iter().for_each(|draw| println!("{:?}", draw));
}
