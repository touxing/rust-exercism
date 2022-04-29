#![allow(unused)]
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
struct Team {
    name: String,
    win: u8,
    loss: u8,
    draw: u8,
}
impl Team {
    fn new(name: String, win: u8, loss: u8, draw: u8) -> Self {
        Self {
            name,
            win,
            loss,
            draw,
        }
    }
    fn update(&mut self, win: u8, loss: u8, draw: u8) {
        self.win += win;
        self.loss += loss;
        self.draw += draw;
    }
}

fn calc_score(
    team: &mut Vec<Team>,
    item1: String,
    item2: String,
    score: (u8, u8, u8),
    score2: (u8, u8, u8),
) {
    if let Some(v) = team.iter_mut().find(|a| a.name == item1) {
        v.update(score.0, score.1, score.2);
    } else {
        team.push(Team::new(item1, score.0, score.1, score.2));
    }
    if let Some(v) = team.iter_mut().find(|a| a.name == item2) {
        v.update(score2.0, score2.1, score2.2);
    } else {
        team.push(Team::new(item2, score2.0, score2.1, score2.2));
    }
}
pub fn tally(match_results: &str) -> String {
    // HashMap 顺序不确定，需要换成 Vec
    let mut team: Vec<Team> = Vec::new();
    let mut result = "Team                           | MP |  W |  D |  L |  P".to_string();

    let v: Vec<&str> = match_results.split('\n').collect();
    for line in v {
        let line_match: Vec<&str> = line.split(';').collect();
        if line_match.len() < 2 {
            continue;
        }
        let item1 = line_match[0].to_string();
        let item2 = line_match[1].to_string();
        if let Some(a) = line_match.last() {
            match *a {
                "win" => {
                    calc_score(
                        &mut team,
                        item1.clone(),
                        item2.clone(),
                        (1, 0, 0),
                        (0, 1, 0),
                    );
                }
                "loss" => {
                    calc_score(
                        &mut team,
                        item1.clone(),
                        item2.clone(),
                        (0, 1, 0),
                        (1, 0, 0),
                    );
                }
                "draw" => {
                    calc_score(
                        &mut team,
                        item1.clone(),
                        item2.clone(),
                        (0, 0, 1),
                        (0, 0, 1),
                    );
                }
                _ => {}
            }
        }
    }
    // socre big in fornt.
    // 问题出在这里，首先根据 point 排序，相等情况下，根据 team 字母排序
    // I don't know then_with function.
    team.sort_by(|a, b| {
        (b.win + b.draw)
            .cmp(&(a.win + a.draw))
            .then_with(|| a.name.cmp(&b.name))
    });
    for value in &team {
        result += &format!(
            "\n{:<31}|  {} |  {} |  {} |  {} |  {}",
            value.name,
            value.win + value.loss + value.draw,
            value.win,
            value.draw,
            value.loss,
            (value.win * 3 + value.draw)
        )
    }
    // println!("{:?}", &team);
    // println!("{:?}", result);
    result
}
