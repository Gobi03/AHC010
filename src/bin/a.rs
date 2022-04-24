#[allow(unused_imports)]
use proconio::marker::{Chars, Isize1, Usize1};
use proconio::{fastout, input};

#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(unused_imports)]
use rand::rngs::ThreadRng;
#[allow(unused_imports)]
use rand::seq::SliceRandom;
#[allow(unused_imports)]
use rand::{thread_rng, Rng};
#[allow(unused_imports)]
use std::io::Write;
use std::time::SystemTime;

#[allow(dead_code)]
const MOD: usize = 1e9 as usize + 7;

const SIDE: usize = 30;

const ROTATE: [usize; 8] = [1, 2, 3, 0, 5, 4, 7, 6];

const TO: [[usize; 4]; 8] = [
    [1, 0, !0, !0],
    [3, !0, !0, 0],
    [!0, !0, 3, 2],
    [!0, 2, 1, !0],
    [1, 0, 3, 2],
    [3, 2, 1, 0],
    [2, !0, 0, !0],
    [!0, 3, !0, 1],
];

const BEAM_WIDTH: usize = 1_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    x: isize,
    y: isize,
}

#[allow(dead_code)]
impl Coord {
    fn new(p: (isize, isize)) -> Self {
        Coord { x: p.0, y: p.1 }
    }
    fn from_usize_pair(p: (usize, usize)) -> Self {
        Coord {
            x: p.0 as isize,
            y: p.1 as isize,
        }
    }

    fn in_field(&self) -> bool {
        (0 <= self.x && self.x < SIDE as isize) && (0 <= self.y && self.y < SIDE as isize)
    }

    // ペアへの変換
    fn to_pair(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    // マンハッタン距離
    fn distance(&self, that: &Self) -> isize {
        (self.x - that.x).abs() + (self.y - that.y).abs()
    }

    fn mk_4dir(&self) -> Vec<Self> {
        let delta = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        delta
            .iter()
            .map(|&p| self.plus(&Coord::new(p)))
            .filter(|&pos| pos.in_field())
            .collect()
    }

    fn com_to_delta(com: char) -> Self {
        match com {
            'U' => Coord::new((0, -1)),
            'D' => Coord::new((0, 1)),
            'L' => Coord::new((-1, 0)),
            'R' => Coord::new((1, 0)),
            _ => unreachable!(),
        }
    }

    // 四則演算
    fn plus(&self, that: &Self) -> Self {
        Coord::new((self.x + that.x, self.y + that.y))
    }
    fn minus(&self, that: &Self) -> Self {
        Coord::new((self.x - that.x, self.y - that.y))
    }

    fn access_matrix<'a, T>(&'a self, mat: &'a Vec<Vec<T>>) -> &'a T {
        &mat[self.y as usize][self.x as usize]
    }

    fn set_matrix<T>(&self, mat: &mut Vec<Vec<T>>, e: T) {
        mat[self.y as usize][self.x as usize] = e;
    }

    // user define
    fn move_to_dir(&self, dir: usize) -> Coord {
        match dir {
            0 => self.plus(&Coord::new((-1, 0))),
            1 => self.plus(&Coord::new((0, -1))),
            2 => self.plus(&Coord::new((1, 0))),
            3 => self.plus(&Coord::new((0, 1))),
            _ => unreachable!(),
        }
    }
}

struct Input {
    t: Vec<Vec<usize>>,
}
impl Input {
    fn new(t: Vec<Vec<char>>) -> Self {
        Self {
            t: t.iter()
                .map(|cs| cs.iter().map(|&c| c as usize - 48).collect())
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Cursor {
    pos: Coord,
    from: usize, // 左上右下 (0123)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    cursor: Cursor,
    ans: Vec<Vec<usize>>,
    mode: usize, // 0~3 角を曲がるごとにインクリメント, 4: 合流を目指す
}
impl State {
    fn new(cursor: Cursor) -> Self {
        Self {
            cursor,
            ans: vec![vec![!0; SIDE]; SIDE],
            mode: 0,
        }
    }

    // 大きいほどよい
    fn eval(&self) -> (usize, usize) {
        let pos_point = match self.mode {
            0 => self.cursor.pos.y as usize,
            1 => self.cursor.pos.x as usize,
            2 => SIDE - self.cursor.pos.y as usize,
            3 | 4 => SIDE - self.cursor.pos.x as usize,
            5 => std::usize::MAX,
            _ => unreachable!(),
        };

        (self.mode, pos_point)
    }

    fn try_to_change_mode(&mut self) {
        match self.mode {
            0 => {
                if self.cursor.pos.y as usize >= SIDE - 2 {
                    self.mode += 1;
                }
            }
            1 => {
                if self.cursor.pos.x as usize >= SIDE - 2 {
                    self.mode += 1;
                }
            }
            2 => {
                if self.cursor.pos.y as usize <= 2 {
                    self.mode += 1;
                }
            }
            3 => {
                if self.cursor.pos.x as usize <= 4 {
                    self.mode += 1;
                }
            }
            4 => (),
            _ => unreachable!(),
        }
    }

    // valid に行けるなら、next_stackに突っ込む
    fn try_go_to(&self, to: usize, rotate_num: usize, input: &Input) -> Option<State> {
        if to != !0 {
            let to_pos = self.cursor.pos.move_to_dir(to);
            if to_pos.in_field() {
                // 合流できるか
                if self.mode == 4 && *to_pos.access_matrix(&self.ans) != !0 {
                    let mut tile = to_pos.access_matrix(&input.t).clone();
                    for i in 0..*to_pos.access_matrix(&self.ans) {
                        tile = ROTATE[tile];
                    }

                    let from = (to + 2) % 4;
                    if TO[tile][from] != 0 {
                        let mut next_st = self.clone();

                        self.cursor.pos.set_matrix(&mut next_st.ans, rotate_num);
                        next_st.cursor = Cursor {
                            pos: to_pos,
                            from: (to + 2) % 4,
                        };

                        next_st.mode = 5;

                        return Some(next_st);
                    }
                }

                if *to_pos.access_matrix(&self.ans) == !0 {
                    let mut next_st = self.clone();

                    self.cursor.pos.set_matrix(&mut next_st.ans, rotate_num);
                    next_st.cursor = Cursor {
                        pos: to_pos,
                        from: (to + 2) % 4,
                    };

                    next_st.try_to_change_mode();

                    return Some(next_st);
                }
            }
        }

        None
    }

    fn print_ans(&self) {
        for y in 0..SIDE {
            for x in 0..SIDE {
                let mut n = self.ans[y][x];
                if n == !0 {
                    n = 0;
                }
                print!("{}", n)
            }
        }
        println!()
    }
}

#[fastout]
fn main() {
    let system_time = SystemTime::now();
    let mut _rng = thread_rng();

    input! {
        t: [Chars; SIDE],
    }

    let input = Input::new(t);

    let sp = Coord::new((2, 2));
    let sc1 = Cursor {
        pos: sp.clone(),
        from: 1,
    };
    let sc2 = Cursor {
        pos: sp.clone(),
        from: 2,
    };

    let mut stack = vec![State::new(sc1), State::new(sc2)];
    // TODO: ループ回数の調整
    loop {
        let mut next_stack = vec![];
        for _ in 0..BEAM_WIDTH {
            if stack.is_empty() {
                break;
            }

            let st = stack.pop().unwrap();

            let mut tile = st.cursor.pos.access_matrix(&input.t).clone();

            // 回転全パターンで次に進む
            let to = TO[tile][st.cursor.from];
            st.try_go_to(to, 0, &input)
                .into_iter()
                .for_each(|next_st| next_stack.push(next_st));

            let rotate_time = if tile < 4 { 3 } else { 1 };
            for i in 1..=rotate_time {
                tile = ROTATE[tile];
                let to = TO[tile][st.cursor.from];
                st.try_go_to(to, i, &input)
                    .into_iter()
                    .for_each(|next_st| next_stack.push(next_st));
            }
        }

        // eprintln!("{}", next_stack.len());

        next_stack.sort_by(|st1, st2| st1.eval().cmp(&st2.eval()));
        eprintln!("{:?}", next_stack[next_stack.len() - 1].cursor);
        stack = next_stack;

        if stack[stack.len() - 1].mode == 5 {
            break;
        }
    }

    stack[stack.len() - 1].print_ans();

    eprintln!("{}ms", system_time.elapsed().unwrap().as_millis());
}
