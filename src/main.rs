// SPDX-License-Identifier: Apache-2.0
// Copyright 2022 Jannik Birk

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    /// Show the results of the matchday instead of the table.
    day: bool,
}

#[derive(Debug,Clone,serde::Deserialize)]
struct MatchResults {
    #[serde(rename(deserialize = "PointsTeam1"))]
    points_home: i32,
    #[serde(rename(deserialize = "PointsTeam2"))]
    points_away: i32,
}

#[derive(Debug,Clone,serde::Deserialize)]
struct Team {
    #[serde(rename(deserialize = "TeamName"))]
    teamname: String,
    #[serde(rename(deserialize = "ShortName"))]
    shortname: String,
}

impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.teamname)
    }
}

#[derive(Debug,Clone,serde::Deserialize)]
struct Match {
    #[serde(rename(deserialize = "Team1"))]
    team_home: Team,
    #[serde(rename(deserialize = "Team2"))]
    team_away: Team,
    #[serde(rename(deserialize = "MatchResults"))]
    results: Vec<MatchResults>,
}

impl std::fmt::Display for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        if self.results.len() > 0 {
            let result = &self.results[0];
            write!(f, "{:>15} [{}:{}] {:15}",
                   self.team_home.shortname,
                   self.team_away.shortname,
                   result.points_home,
                   result.points_away)
        } else {
            write!(f, "{:>15}  vs.  {:15}",
                   self.team_home.shortname,
                   self.team_away.shortname)
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
struct TableEntry {
    #[serde(rename(deserialize = "TeamName"))]
    teamname: String,
    #[serde(rename(deserialize = "Points"))]
    points: i32,
    #[serde(rename(deserialize = "Won"))]
    wins: i32,
    #[serde(rename(deserialize = "Draw"))]
    draws: i32,
    #[serde(rename(deserialize = "Lost"))]
    lost: i32,
    #[serde(rename(deserialize = "Goals"))]
    goals: i32,
    #[serde(rename(deserialize = "OpponentGoals"))]
    goals_opp: i32,
}

impl std::fmt::Display for TableEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:24} {}:{}:{}\t{:2}:{:<2} {}",
               self.teamname,
               self.wins,
               self.draws,
               self.lost,
               self.goals,
               self.goals_opp,
               self.points)
    }
}

fn print_table(table: &Vec<TableEntry>) {
    println!("    Team                     W:D:L\tG     P");
    for (i,e) in table.iter().rev().enumerate() {
        println!("{:2}. {}", i, e);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.day {
        let data = reqwest::blocking::get("https://www.openligadb.de/api/getmatchdata/bl1")?
            .json::<Vec<Match>>()?;

        for mmatch in data {
            println!("{}", mmatch);
        }

        return Ok(())
    }

    let mut table = reqwest::blocking::get("https://www.openligadb.de/api/getbltable/bl1/2022")?
        .json::<Vec<TableEntry>>()?;
    table.sort_by(|a, b| a.points.cmp(&b.points));
    print_table(&table);

    Ok(())
}
