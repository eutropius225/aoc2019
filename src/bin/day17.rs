use aoc::intcode::{Program, VM};
use itertools::Itertools;
use aoc::util::DIRECTIONS;

#[allow(unused)]
fn display_ascii(img: &Vec<Vec<char>>) {
    println!("{}", img.iter().map(|l| l.iter().join("")).join("\n"));
}

fn main() {
    let ascii = Program::from_stdin().unwrap();
    let mut img = VM::of(&ascii)
        .map(|i| i as u8 as char)
        .join("")
        .trim()
        .split("\n")
        .map(|s| s.chars().collect_vec())
        .collect_vec();

    let height = img.len();
    let width = img[0].len();
    let mut alignment = 0;

    for y in 0..height as i32 {
        for x in 0..width as i32 {
            if img[y as usize][x as usize] != '#' {
                continue;
            }
            let mut intersection = true;
            for dir in &DIRECTIONS {
                let (nx, ny) = dir.offset((x, y));
                if nx < 0 || ny < 0 || *img
                    .get(ny as usize)
                    .and_then(|v| v.get(nx as usize))
                    .unwrap_or(&'.') == '.'
                {
                    intersection = false;
                    break;
                }
            }
            if intersection {
                img[y as usize][x as usize] = 'O';
                alignment += x * y;
            }
        }
    }

    // display_ascii(&img);
    println!("Alignment: {}", alignment);

    let mut vm = VM::of(&ascii);
    vm.mem[0] = 2;
    // ......#############...........................#############
    // ......#...........#...........................#...........#
    // ......#...........#...........................#...........#
    // ......#...........#...........................#...........#
    // ......#...........#...........................#...........#
    // ......#...........#...........................#...........#
    // ......#...........#.#########.........########O##.........#
    // ......#...........#.#.......#.........#.......#.#.........#
    // ......#...........##O#######O##.......#.......#.#.........#
    // ......#.............#.......#.#.......#.......#.#.........#
    // ......#.............#.......#.#.....##O########.#.........#
    // ......#.............#.......#.#.....#.#.........#.........#
    // ......###########...#.......#.#.....#.#.........#.#########
    // ................#...#.......#.#.....#.#.........#.#........
    // ................#...#.......##O#####O#O##.......#.#........
    // ................#...#.........#.....#.#.#.......#.#........
    // ................#...#.........######O##.#.......##O########
    // ................#...#...............#...#.........#.......#
    // ............^###O####...............#...#.........#.......#
    // ................#...................#...#.........#.......#
    // ................#...................####O######...#.......#
    // ................#.......................#.....#...#.......#
    // ........#########.......................######O####.......#
    // ........#.....................................#...........#
    // ........#.....................................#...........#
    // ........#.....................................#...........#
    // ........#.....................................#...........#
    // ........#.....................................#...........#
    // ........#.....................................#############
    // ........#..................................................
    // ........#..................................................
    // ........#..................................................
    // ........#..................................................
    // ........#..................................................
    // #########..................................................

    // A----------- A----------- C------------ B------------------ C------------ B------------------ C------------ A----------- B------------------ A-----------
    // R,8,L,12,R,8,R,8,L,12,R,8,L,10,L,10,R,8,L,12,L,12,L,10,R,10,L,10,L,10,R,8,L,12,L,12,L,10,R,10,L,10,L,10,R,8,R,8,L,12,R,8,L,12,L,12,L,10,R,10,R,8,L,12,R,8
    vm.input_ascii("A,A,C,B,C,B,C,A,B,A\n");
    vm.input_ascii("R,8,L,12,R,8\n"); // A
    vm.input_ascii("L,12,L,12,L,10,R,10\n"); // B
    vm.input_ascii("L,10,L,10,R,8\n"); // C
    vm.input_ascii("n\n");
    println!("Dust: {}", vm.last().unwrap());
}
