use std::fmt::Debug;

struct Draw {
    blue: u8,
    green: u8,
    red: u8,
}

impl Debug for Draw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.blue > 0 {
            write!(f, "{} blue, ", self.blue)?;
        }
        if self.green > 0 {
            write!(f, "{} green, ", self.green)?;
        }
        if self.red > 0 {
            write!(f, "{} red, ", self.red)?;
        }
        Ok(())
    }
}

fn main() {
    let game = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
    let (game_id, s) = game.split_once(':')
        .map(|(id, s)| (
            id.trim().split_ascii_whitespace().nth(1).unwrap().parse::<u16>().unwrap(),
            s.split(';')
                .map(|x| x.split(',')
                    .map(|x| x.trim())
                    .map(|x| x.split_once(' ').map(|(n, color)| (n.parse::<u8>().unwrap(), color)).unwrap())
                    .map(|(n, color)|
                        match color {
                            "red" => Draw { blue: 0, green: 0, red: n },
                            "blue" => Draw { blue: n, green: 0, red: 0 },
                            "green" => Draw { blue: 0, green: n, red: 0 },
                            _ => panic!("Unknown color: {color}"),
                        })
                    .fold(Draw { blue: 0, green: 0, red: 0 }, |acc, draw| Draw {
                        blue: acc.blue + draw.blue,
                        green: acc.green + draw.green,
                        red: acc.red + draw.red,
                    })
                )
                .collect::<Vec<_>>()
        )).unwrap();

    assert_eq!(game_id, 3);
    println!("{:?}", game_id, );
    s.iter().for_each(|draw| println!("{:?}", draw));
}
