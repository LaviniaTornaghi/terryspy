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

struct Person {
    username: String,
    tasks: Vec<Task>,
}

const MAX_ID_SIZE: usize = 15;

fn trim_id(id: &str) -> String {
    if id.len() > MAX_ID_SIZE {
        format!("{}...", &id[..MAX_ID_SIZE - 3]) 
    } else {
        id.to_owned()
    }
}

fn score_str(score: f64, total: f64) -> ColoredString {
    if score == 0. {
        format!("{score:.0}/{total:.0}").red()
    } else if score == total {
        format!("{score:.0}/{total:.0}").green()
    } else {
        format!("{score:.0}/{total:.0}").yellow()
    }
}

fn print_people(people: &[Person]) {
    let task_no = people[0].tasks.len();

    // prints usernames
    print!(" {:>width$} |", "", width = MAX_ID_SIZE);
    for Person { username, .. } in people {
        print!(" {:>width$} |", trim_id(username), width = MAX_ID_SIZE);
    }
    println!();

    //prints empty line
    print!("{:->width$}|", "", width = MAX_ID_SIZE + 2);
    for _ in people {
        print!("{:->width$}|", "", width = MAX_ID_SIZE + 2);
    }
    println!();

    // prints total scores
    print!(" {:>width$} |", "Total", width = MAX_ID_SIZE);
    for Person { tasks, .. } in people {
        let (score, total_score) = tasks.iter().fold((0.0, 0.0), |(score, total_score), task| {
            (score + task.score, total_score + task.max_score)
        });

        print!(" {:>width$} |", score_str(score, total_score), width = MAX_ID_SIZE);
    }
    println!();
    
    //prints empty line
    print!("{:->width$}|", "", width = MAX_ID_SIZE + 2);
    for _ in people {
        print!("{:->width$}|", "", width = MAX_ID_SIZE + 2);
    }
    println!();

    // prints scores
    for i in 0..task_no {
        print!(" {:>width$} |", trim_id(&people[0].tasks[i].name), width = MAX_ID_SIZE);
        for Person { tasks, .. } in people {
            print!(" {:>width$} |", score_str(tasks[i].score, tasks[i].max_score), width = MAX_ID_SIZE);
        }
        println!();
    }
}

fn query(username: &str) -> error::Result<Person> {
    let tasks: Vec<Task> = reqwest::blocking::get(format!("https://territoriali.olinfo.it/api/user/{username}/scores"))?.json()?;
    
    if tasks.len() == 0 {
        Err(error::Error::from(format!("User '{username}' does not exist!")))
    } else {
        Ok(Person { username: username.to_string(), tasks })
    }
}

fn main() -> error::Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        return Err(error::Error::from("No arguments given!"));
    }

    let people = args[1..].iter().map(|username| query(username)).collect::<Result<Vec<_>, _>>()?;
    print_people(&people);

    Ok(())
}
