/*  Copyright 2024 Francesco Vercellesi, Lavinia Tornaghi
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 */

mod error;

use std::env;
use serde::Deserialize;
use colored::*;

#[derive(Debug, Deserialize)]
struct Task {
    title: String,
    name: String,
    score: f64,
    max_score: f64,
}

type Person = Vec<Task>;

fn print_person(username: &str, person: &Person) {
    let (score, total_score) = person.iter().fold((0.0, 0.0), |(score, total_score), task| {
        (score + task.score, total_score + task.max_score)
    });

    let total_str = format!("{score}/{total_score}");
    if score == total_score {
        println!("{}: {}", username, total_str.green());
    } else if score == 0. {
        println!("{}: {}", username, total_str.red());
    } else {
        println!("{}: {}", username, total_str.yellow());
    }

    for Task { name, score, max_score, .. } in person {
        let total_str = format!("{score}/{max_score}");

        if score == max_score {
            println!("{:>20}: {}", name, total_str.green());
        } else if score == &0. {
            println!("{:>20}: {}", name, total_str.red());
        } else {
            println!("{:>20}: {}", name, total_str.yellow());
        }
    }
}

fn query(username: &str) -> error::Result<()> {
    let person: Person = reqwest::blocking::get(format!("https://territoriali.olinfo.it/api/user/{username}/scores"))?.json()?;
    
    print_person(username, &person);

    Ok(())
}

fn main() -> error::Result<()> {
    let args: Vec<_> = env::args().collect();

    for username in &args[1..] {
        query(username)?;
    }

    Ok(())
}
