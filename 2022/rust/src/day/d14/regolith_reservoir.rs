use crate::day::{Input, InputError, InputResult, Output, Pos, Wrapper};
use std::collections::HashSet;

#[derive(PartialEq, Eq)]
enum State {
    Stopped,
    Continue,
    Infinite,
}

fn solve(cave: &HashSet<Pos>, bottom: i32) -> Output {
    let mut resting: u32 = 0;
    let mut sand: HashSet<Pos> = HashSet::new();

    'outer: loop {
        let (mut x, mut y) = (500, 0);
        let mut state: State = State::Continue;

        while state == State::Continue {
            state = match drop_sand(cave, &mut sand, bottom, &mut x, &mut y) {
                State::Stopped => {
                    if x == 500 && y == 0 {
                        return Output::Nu32(resting + 1);
                    }

                    resting += 1;
                    sand.insert((x, y));
                    State::Stopped
                }
                State::Continue => State::Continue,
                State::Infinite => break 'outer,
            };
        }
    }

    Output::Nu32(resting)
}

fn drop_sand(
    cave: &HashSet<Pos>,
    sand: &mut HashSet<Pos>,
    bottom: i32,
    x: &mut i32,
    y: &mut i32,
) -> State {
    if try_down(cave, &sand, *x, y, bottom) {
        if try_left(cave, sand, x, *y) || try_right(cave, sand, x, *y) {
            return State::Continue;
        }
    } else {
        return State::Infinite;
    }

    State::Stopped
}

fn try_down(cave: &HashSet<Pos>, sand: &HashSet<Pos>, x: i32, y: &mut i32, bottom: i32) -> bool {
    while *y < bottom && !cave.contains(&(x, *y + 1)) && !sand.contains(&(x, *y + 1)) {
        *y += 1;
    }

    *y < bottom
}

fn try_left(cave: &HashSet<Pos>, sand: &HashSet<Pos>, x: &mut i32, y: i32) -> bool {
    if cave.contains(&(*x - 1, y + 1)) || sand.contains(&(*x - 1, y + 1)) {
        return false;
    }

    *x -= 1;
    true
}

fn try_right(cave: &HashSet<Pos>, sand: &HashSet<Pos>, x: &mut i32, y: i32) -> bool {
    if cave.contains(&(*x + 1, y + 1)) || sand.contains(&(*x + 1, y + 1)) {
        return false;
    }

    *x += 1;
    true
}

pub fn run(input: Input) -> (Output, Output) {
    let (mut cave, bottom): (HashSet<Pos>, i32) = input.unwrap();

    let part_one = solve(&cave, bottom);

    for x in -1000..1000 {
        cave.insert((x, bottom + 2));
    }

    (part_one, solve(&cave, bottom + 2))
}

pub fn parse() -> InputResult<Input> {
    let mut cave = HashSet::new();
    let mut bottom: i32 = 0;

    for line in std::fs::read_to_string("../input/14")?.lines() {
        let mut values = Vec::<(i32, i32)>::new();

        for element in line.split(" -> ").collect::<Vec<&str>>().iter() {
            values.push(if let Some((x, y)) = element.split_once(',') {
                (x.parse()?, y.parse()?)
            } else {
                return Err(InputError::InvalidInput);
            });
        }

        for i in 0..values.len().saturating_sub(1) {
            for x in values[i].0.min(values[i + 1].0)..=values[i].0.max(values[i + 1].0) {
                let (from, to) = if values[i].1 < values[i + 1].1 {
                    (values[i].1, values[i + 1].1)
                } else {
                    (values[i + 1].1, values[i].1)
                };

                for y in from..=to {
                    cave.insert((x, y));
                }

                bottom = bottom.max(to);
            }
        }
    }

    Ok(Input::D14((cave, bottom)))
}
