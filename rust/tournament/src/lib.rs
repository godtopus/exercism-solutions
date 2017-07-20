use std::collections::HashMap;
use std::cmp::Ordering;

struct TeamResult {
    wins: u32,
    draws: u32,
    losses: u32
}

impl TeamResult {
    pub fn points(&self) -> u32 {
        self.wins * 3 + self.draws
    }
}

pub fn tally(results: &str) -> String {
    let mut teams = HashMap::new();
    let table = vec!["Team                           | MP |  W |  D |  L |  P".to_owned()];
    if results.is_empty() {
        return table.join("")
    }

    for result in results.split('\n') {
        let parts: Vec<&str>  = result.split(';').collect();
        match parts[2] {
            "win" => {
                (*teams.entry(parts[0]).or_insert(TeamResult { wins: 0, draws: 0, losses: 0 })).wins += 1;
                (*teams.entry(parts[1]).or_insert(TeamResult { wins: 0, draws: 0, losses: 0 })).losses += 1;
            },
            "draw" => {
                (*teams.entry(parts[0]).or_insert(TeamResult { wins: 0, draws: 0, losses: 0 })).draws += 1;
                (*teams.entry(parts[1]).or_insert(TeamResult { wins: 0, draws: 0, losses: 0 })).draws += 1;
            },
            "loss" => {
                (*teams.entry(parts[0]).or_insert(TeamResult { wins: 0, draws: 0, losses: 0 })).losses += 1;
                (*teams.entry(parts[1]).or_insert(TeamResult { wins: 0, draws: 0, losses: 0 })).wins += 1;
            },
            _ => ()
        }
    }

    let mut teams = teams.iter().collect::<Vec<(&&str, &TeamResult)>>();
    teams.sort_by(|&t1, &t2| {
        match (t1, t2) {
            ((_, r1), (_, r2)) if r1.points() > r2.points() => Ordering::Less,
            ((_, r1), (_, r2)) if r1.points() < r2.points() => Ordering::Greater,
            ((team1, _), (team2, _)) if team1 > team2 => Ordering::Greater,
            ((team1, _), (team2, _)) if team1 < team2 => Ordering::Less,
            _ => Ordering::Equal
        }
    });

    teams.iter().fold(table, |mut acc, &(team, r)| {
        let games = r.wins + r.draws + r.losses;
        let points = r.wins * 3 + r.draws;
        acc.push(format!("{:30} | {:2} | {:2} | {:2} | {:2} | {:2}", team, games, r.wins, r.draws, r.losses, points).to_owned());
        acc
    }).join("\n")
}