
mod database;
mod cli;

use std::{ops::RangeInclusive, rc::Rc};

use clap::Parser;
use cli::Cli;
use faker_rand::en_us::names::{LastName, FirstName};
use rand::Rng;
use chrono::{DateTime, Datelike, Utc};


const RATING_RANGE: RangeInclusive<u16> = 1000..=2900;

#[tokio::main]
async fn main() {

    let cli = Cli::parse();

    // database::connect().await.ok();
    let players: Rc<Vec<_>> = Rc::new((0..10).map(|i| Player::rand(i)).collect::<Vec<_>>());
    
    let date = Utc::now();
    
    let tournament = Tournament {
        max_participants_num: players.len() as u32,
        month: date.month(),
        year: date.year(),
        day: date.day(),
        tours_amount: 7,
    };
    
    // match draw {
    //     DrawSystem::RoundRobin => todo!(),
    //     DrawSystem::Swiss => todo!(),
    // }

    // dbg!("{:?}", shuffle(players.to_vec()));
    // dbg!("{:?}", Result::Win.value());
}


/// Fisher–Yates shuffle
fn shuffle<T>(vec: Vec<T>) -> Vec<T> {
    let mut result = vec;
    for i in (1..result.len()-1).rev() {
        result.swap(rand::thread_rng().gen_range(0..i), i);
    }
    result.into()
}

struct Tour {
    start_time: Utc,
    end_time: Utc,
    pairs: Vec<Pair>
}

#[derive(Debug)]
struct Tournament {
    year: i32,
    month: u32,
    day: u32,
    tours_amount: u32,
    max_participants_num: u32,
}


#[derive(Debug)]
enum Result {
    Win,
    Loss,
    Draw
}

struct Participant {
    result: Result
}

struct Pair {
    table_num: u32,
    white: Participant,
    black: Participant,
}

enum DrawSystem {
    RoundRobin,
    Swiss
}

enum ChessDiscipline {
    Blitz,
    Rapid,
    Classic
}

#[derive(Debug, Clone)]
struct Rating {
    blitz: Option<u16>,
    rapid: Option<u16>,
    standart: Option<u16>
}

#[derive(Debug, Clone)]
/// Системы подсчета шахматного рейтинга
struct Ratings {
    /// FIDE: https://ratings.fide.com/
    fide: Option<Rating>, 
    /// ФШР: https://ratings.ruchess.ru/
    fsr: Option<Rating>   
}

#[derive(Debug, Clone)]
struct Player {
    num: usize,
    surname: String,
    name: String,
    patronymic: Option<String>,
    rating: Option<Ratings>
}

impl Result {
    fn value(&self) -> f32 {
        match self {
            Result::Win => 1.0,
            Result::Loss => 0.0,
            Result::Draw => 0.5,
        }
    }
}

impl Player {
    fn rand(number: usize) -> Self {
        Self {
            num: number,
            surname: rand::random::<LastName>().to_string(),
            name: rand::random::<FirstName>().to_string(),
            patronymic: None,
            rating: Some(Ratings { fide: None, fsr: Some(Rating { 
                blitz: Some(rand::thread_rng().gen_range(RATING_RANGE)), 
                rapid: Some(rand::thread_rng().gen_range(RATING_RANGE)), 
                standart: Some(rand::thread_rng().gen_range(RATING_RANGE)) 
            }) }),
        }
    }
}


