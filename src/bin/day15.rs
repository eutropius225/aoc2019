use aoc::intcode::{Program, VM, Int, State};
use std::collections::{VecDeque, HashSet, HashMap};
use std::mem::swap;

#[derive(Clone, Copy)]
enum Dir {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Dir {
    fn offset(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            Dir::North => (x, y + 1),
            Dir::South => (x, y - 1),
            Dir::West => (x - 1, y),
            Dir::East => (x + 1, y),
        }
    }
}

enum Tile {
    Wall = 0,
    Space = 1,
    Oxygen = 2,
}

static DIRECTIONS: [Dir; 4] = [Dir::North, Dir::South, Dir::West, Dir::East];

#[allow(unused)]
fn print_positions(positions: &HashMap<(i32, i32), Tile>) {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for (&(x, y), _) in positions {
        if x < min_x { min_x = x; }
        if x > max_x { max_x = x; }
        if y < min_y { min_y = y; }
        if y > max_y { max_y = y; }
    }

    for y in min_y..(max_y + 1) {
        for x in min_x..(max_x + 1) {
            print!("{}",
                   match positions.get(&(x, y)) {
                       None => ' ',
                       Some(t) => match t {
                           Tile::Wall => '#',
                           Tile::Space => '.',
                           Tile::Oxygen => 'O',
                       }
                   }
            )
        }
        println!();
    }
    println!()
}

fn main() {
    let program = Program::from_stdin().unwrap();

    let mut droids = VecDeque::new();
    let mut tq = VecDeque::new();
    let mut seen = HashSet::new();
    droids.push_back(((0, 0), VM::of(&program)));
    seen.insert((0, 0));
    let mut steps = 0;

    let mut positions = HashMap::new();
    let mut oxygen_source = (0, 0);

    while !droids.is_empty() {
        steps += 1;
        for (pos, vm) in &droids {
            for &dir in &DIRECTIONS {
                let new_pos = dir.offset(*pos);
                if seen.insert(new_pos) {
                    let mut new_vm = vm.clone();
                    new_vm.input(dir as Int);
                    match new_vm.next_state().unwrap() {
                        State::Outputting(status) => {
                            positions.insert(
                                new_pos,
                                match status {
                                    0 => Tile::Wall,
                                    1 => {
                                        tq.push_back((new_pos, new_vm));
                                        Tile::Space
                                    }
                                    2 => {
                                        println!("Steps: {}", steps);
                                        oxygen_source = new_pos;
                                        Tile::Oxygen
                                    }
                                    _ => panic!(),
                                });
                        }
                        _ => panic!(),
                    }
                }
            }
        }
        droids.clear();
        swap(&mut droids, &mut tq);
    }
    tq.clear();

    let mut oxq = VecDeque::new();
    let mut oxqt = VecDeque::new();
    let mut minutes = 0;
    oxq.push_back(oxygen_source);

    loop {
        for &pos in &oxq {
            for &dir in &DIRECTIONS {
                let new_pos = dir.offset(pos);
                match positions.get(&new_pos) {
                    Some(Tile::Space) => {
                        positions.insert(new_pos, Tile::Oxygen);
                        oxqt.push_back(new_pos);
                    }
                    _ => {}
                }
            }
        }
        oxq.clear();
        swap(&mut oxq, &mut oxqt);
        if oxq.is_empty() {
            break;
        }
        minutes += 1;
    }
    println!("Minutes: {}", minutes);
}