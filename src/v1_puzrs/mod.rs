use std::ops::Index;

mod solver2;
mod util;

pub use self::solver2::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clue(pub i32);

pub const NO_CLUE: Clue = Clue(0);
pub const UNUSED: Clue = Clue(-1);

use util::{D, FOUR_NEIGHBOURS, Grid, LP, P};

#[derive(Clone)]
pub struct LinePlacement {
    right: Grid<bool>,
    down: Grid<bool>,
}

impl LinePlacement {
    pub fn new(height: i32, width: i32) -> LinePlacement {
        LinePlacement {
            right: Grid::new(height, width - 1, false),
            down: Grid::new(height - 1, width, false),
        }
    }
    pub fn height(&self) -> i32 {
        self.right.height()
    }
    pub fn width(&self) -> i32 {
        self.down.width()
    }
    pub fn right(&self, pos: P) -> bool {
        self.right.is_valid_p(pos) && self.right[pos]
    }
    pub fn set_right(&mut self, pos: P, e: bool) {
        self.right[pos] = e;
    }
    pub fn down(&self, pos: P) -> bool {
        self.down.is_valid_p(pos) && self.down[pos]
    }
    pub fn set_down(&mut self, pos: P, e: bool) {
        self.down[pos] = e;
    }
    pub fn get(&self, pos: LP) -> bool {
        let LP(y, x) = pos;
        match (y % 2, x % 2) {
            (0, 1) => self.right(P(y / 2, x / 2)),
            (1, 0) => self.down(P(y / 2, x / 2)),
            _ => panic!(),
        }
    }
    pub fn get_checked(&self, pos: LP) -> bool {
        let LP(y, x) = pos;
        if 0 <= y && y < self.height() * 2 - 1 && 0 <= x && x < self.width() * 2 - 1 {
            self.get(pos)
        } else {
            false
        }
    }
    pub fn isolated(&self, pos: P) -> bool {
        !(self.right(pos + D(0, -1))
            || self.right(pos)
            || self.down(pos + D(-1, 0))
            || self.down(pos))
    }
    pub fn is_endpoint(&self, pos: P) -> bool {
        let mut n_lines = 0;
        let pos_vtx = LP::of_vertex(pos);
        for &d in &FOUR_NEIGHBOURS {
            if self.get_checked(pos_vtx + d) {
                n_lines += 1;
            }
        }
        n_lines == 1
    }
    pub fn extract_chain_groups(&self) -> Option<Grid<i32>> {
        let height = self.height();
        let width = self.width();
        let mut ids = Grid::new(height, width, -1);
        let mut last_id = 0;

        for y in 0..height {
            for x in 0..width {
                let pos = P(y, x);
                if self.is_endpoint(pos) && ids[pos] == -1 {
                    // traverse chain
                    let mut l = P(-1, -1);
                    let mut c = pos;

                    'traverse: loop {
                        ids[c] = last_id;
                        for &d in &FOUR_NEIGHBOURS {
                            if c + d != l && self.get_checked(LP::of_vertex(c) + d) {
                                l = c;
                                c = c + d;
                                continue 'traverse;
                            }
                        }
                        break;
                    }
                    last_id += 1;
                }
            }
        }

        for y in 0..height {
            for x in 0..width {
                let pos = P(y, x);
                if ids[pos] == -1 {
                    return None;
                }
                if y < height - 1 && (ids[pos] == ids[pos + D(1, 0)]) != self.down(pos) {
                    return None;
                }
                if x < width - 1 && (ids[pos] == ids[pos + D(0, 1)]) != self.right(pos) {
                    return None;
                }
            }
        }

        Some(ids)
    }
}

pub struct AnswerDetail {
    pub answers: Vec<LinePlacement>,
    pub fully_checked: bool,
    pub found_not_fully_filled: bool,
    pub n_steps: u64,
}
impl AnswerDetail {
    pub fn len(&self) -> usize {
        self.answers.len()
    }
}
impl Index<usize> for AnswerDetail {
    type Output = LinePlacement;
    fn index(&self, idx: usize) -> &LinePlacement {
        &self.answers[idx]
    }
}

fn answer_common(problem: &[i32], height: i32, width: i32) -> String {
    let mut toks = vec![];
    for y in 0..height {
        for x in 0..width {
            let n = problem[(y * width + x) as usize];
            if 1 <= n {
                toks.push(format!("{{\"y\":{},\"x\":{},\"color\":\"black\",\"item\":{{\"kind\":\"text\",\"data\":\"{}\"}}}}", y * 2 + 1, x * 2 + 1, n));
            }
        }
    }
    format!(
        "{{\"kind\":\"grid\",\"height\":{},\"width\":{},\"defaultStyle\":\"grid\",\"data\":[{}]}}",
        height,
        width,
        &toks.join(",")
    )
}

fn find_extra_answer(ans: &LinePlacement, height: i32, width: i32) -> Option<LinePlacement> {
    let mut unit_id = vec![vec![-1; width as usize]; height as usize];
    let mut next_id = 0;
    for y in 0..height {
        for x in 0..width {
            let p = P(y, x);
            if unit_id[y as usize][x as usize] != -1 {
                continue;
            }
            let mut n_adj = 0;
            for d in &FOUR_NEIGHBOURS {
                if ans.get_checked(LP::of_vertex(p) + *d) {
                    n_adj += 1;
                }
            }

            if n_adj != 1 {
                continue;
            }

            let id = next_id;
            next_id += 1;

            let mut cur = p;
            let mut last = P(-1, -1);
            'trace: loop {
                unit_id[cur.y() as usize][cur.x() as usize] = id;
                for d in &FOUR_NEIGHBOURS {
                    let nex = cur + *d;
                    if nex != last && ans.get_checked(LP::of_vertex(cur) + *d) {
                        last = cur;
                        cur = nex;
                        continue 'trace;
                    }
                }
                break;
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            let id = unit_id[y as usize][x as usize];
            if id == -1 {
                continue;
            }

            // perform BFS to find another chain with the same id that does not pass through cells with id != -1
            let mut visited = vec![vec![false; width as usize]; height as usize];
            let mut pre = vec![vec![P(-1, -1); width as usize]; height as usize];
            let mut q = std::collections::VecDeque::new();
            visited[y as usize][x as usize] = true;
            q.push_back(P(y, x));

            let mut dest = None;

            while let Some(cur) = q.pop_front() {
                if cur != P(y, x) && unit_id[cur.y() as usize][cur.x() as usize] == id {
                    dest = Some(cur);
                    break;
                }

                for d in &FOUR_NEIGHBOURS {
                    let nex = cur + *d;
                    if !(0 <= nex.y() && nex.y() < height && 0 <= nex.x() && nex.x() < width) {
                        continue;
                    }

                    let uid = unit_id[nex.y() as usize][nex.x() as usize];
                    if ans.get_checked(LP::of_vertex(cur) + *d) {
                        continue;
                    }

                    if visited[nex.y() as usize][nex.x() as usize] {
                        continue;
                    }

                    if uid != id && uid != -1 {
                        continue;
                    }

                    visited[nex.y() as usize][nex.x() as usize] = true;
                    pre[nex.y() as usize][nex.x() as usize] = cur;
                    q.push_back(nex);
                }
            }

            if let Some(dest) = dest {
                // reconstruct the path
                let mut ret = LinePlacement::new(height, width);
                let mut cur = dest;
                while pre[cur.y() as usize][cur.x() as usize] != P(-1, -1) {
                    let nex = pre[cur.y() as usize][cur.x() as usize];
                    if cur.y() == nex.y() {
                        // horizontal
                        let y = cur.y();
                        let x = std::cmp::min(cur.x(), nex.x());
                        ret.set_right(P(y, x), true);
                    } else {
                        // vertical
                        let y = std::cmp::min(cur.y(), nex.y());
                        let x = cur.x();
                        ret.set_down(P(y, x), true);
                    }
                    cur = nex;
                }
                return Some(ret);
            }
        }
    }

    None
}

fn answer_to_json(ans: &LinePlacement, height: i32, width: i32) -> String {
    let mut toks = vec![];
    for y in 0..height {
        for x in 0..width {
            if x < width - 1 && ans.right(P(y, x)) {
                toks.push(format!(
                    "{{\"y\":{},\"x\":{},\"color\":\"green\",\"item\":\"line\"}}",
                    y * 2 + 1,
                    x * 2 + 2
                ));
            }
            if y < height - 1 && ans.down(P(y, x)) {
                toks.push(format!(
                    "{{\"y\":{},\"x\":{},\"color\":\"green\",\"item\":\"line\"}}",
                    y * 2 + 2,
                    x * 2 + 1
                ));
            }
        }
    }

    let extra_path = find_extra_answer(ans, height, width);
    if let Some(extra_path) = extra_path {
        for y in 0..height {
            for x in 0..width {
                if x < width - 1 && extra_path.right(P(y, x)) {
                    toks.push(format!(
                        "{{\"y\":{},\"x\":{},\"color\":\"red\",\"item\":\"dottedLine\"}}",
                        y * 2 + 1,
                        x * 2 + 2
                    ));
                }
                if y < height - 1 && extra_path.down(P(y, x)) {
                    toks.push(format!(
                        "{{\"y\":{},\"x\":{},\"color\":\"red\",\"item\":\"dottedLine\"}}",
                        y * 2 + 2,
                        x * 2 + 1
                    ));
                }
            }
        }
    }
    format!(
        "{{\"kind\":\"grid\",\"height\":{},\"width\":{},\"defaultStyle\":\"empty\",\"data\":[{}]}}",
        height,
        width,
        &toks.join(",")
    )
}

pub fn solve_problem(problem: &[i32], height: i32, width: i32, limit: usize) -> String {
    let mut board = Grid::new(height, width, NO_CLUE);
    for y in 0..height {
        for x in 0..width {
            board[P(y, x)] = Clue(problem[(y * width + x) as usize]);
        }
    }
    let res = solve2(&board, Some(limit), false, false).answers;
    let ret_string;
    if res.len() == 0 {
        ret_string = "{\"status\":\"error\",\"description\":\"no answer\"}".to_owned();
    } else {
        let common_json = answer_common(problem, height, width);
        let ans_json = res
            .iter()
            .map(|a| answer_to_json(a, height, width))
            .collect::<Vec<_>>()
            .join(",");
        ret_string = format!(
            "{{\"status\":\"ok\",\"description\":{{\"common\":{},\"answers\":[{}]}}}}",
            common_json, ans_json
        );
    }
    ret_string
}
