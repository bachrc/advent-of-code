use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread::sleep;
use Coup::{Paper, Rock, Scissors};
use MatchIssue::{Draw, Lose, Win};

enum Coup {
    Rock, Paper,
    Scissors
}

impl Coup {
    fn fight(&self, other: Coup) -> MatchIssue {
        return match self {
            Rock =>
                match other {
                    Rock => Draw,
                    Paper => Lose,
                    Scissors => Win
                }
            Paper =>
                match other {
                    Rock => Win,
                    Paper => Draw,
                    Scissors => Lose
                }
            Scissors =>
                match other {
                    Rock => Lose,
                    Paper => Win,
                    Scissors => Draw
                }
        }
    }

    fn points(&self) -> i32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3
        }
    }

}

enum MatchIssue {
    Win, Lose, Draw
}

impl MatchIssue {
    fn obtained_against(&self, other: Coup) -> Coup {
        match self {
            Win => match other {
                Rock => Paper,
                Paper => Scissors,
                Scissors => Rock
            }
            Lose => match other {
                Rock => Scissors,
                Paper => Rock,
                Scissors => Paper
            }
            Draw => other
        }
    }

    fn points(&self) -> i32 {
        match self {
            Lose => 0,
            Draw => 3,
            Win => 6
        }
    }
}

fn parse_coup(coup: &str) -> Result<Coup, &str> {
    match coup {
        "A" | "X" => Ok(Rock),
        "B" | "Y" => Ok(Paper),
        "C" | "Z" => Ok(Scissors),
        _ => Err("Unhandled identifier")
    }
}

fn parse_issue(issue: &str) -> Result<MatchIssue, &str> {
    match issue {
        "X" => Ok(Lose),
        "Y" => Ok(Draw),
        "Z" => Ok(Win),
        _ => Err("Unhandler issue")
    }
}

fn main() {
    let file = File::open("day2/input.txt").expect("file not found!");
    let reader = BufReader::new(&file);

    let mut points = 0;

    for parsed_line in reader.lines() {
        let ehoui = parsed_line.expect("chelou");
        let parts: Vec<&str> = ehoui.split_whitespace().collect();
        let (id_coup_adverse, id_coup_de_moi) = (*parts.get(0).unwrap(), *parts.get(1).unwrap());

        let coup_adverse = parse_coup(id_coup_adverse).expect("chelou le coup de l'adversaire");
        let coup_de_moi = parse_coup(id_coup_de_moi).expect("chelou le coup de moi");

        let issue_du_match = coup_de_moi.fight(coup_adverse);

        points += issue_du_match.points() + coup_de_moi.points();
    }

    println!("Tu as gagné {} points !", points);

    println!("PARTIE 2");

    let file = File::open("day2/input.txt").expect("file not found!");
    let reader = BufReader::new(&file);
    let mut points = 0;

    for parsed_line in reader.lines() {
        let ehoui = parsed_line.expect("chelou");
        let parts: Vec<&str> = ehoui.split_whitespace().collect();
        let (id_coup_adverse, id_coup_de_moi) = (*parts.get(0).unwrap(), *parts.get(1).unwrap());

        let coup_adverse = parse_coup(id_coup_adverse).unwrap();
        let issue_wanted = parse_issue(id_coup_de_moi).unwrap();

        let coup_i_need_to_do = issue_wanted.obtained_against(coup_adverse);

        points += issue_wanted.points() + coup_i_need_to_do.points()
    }

    println!("Tu aurais ce nombre de points : {}", points)
}
