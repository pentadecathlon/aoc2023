use crate::timed;
use core::simd::*;
pub fn solve() {
    println!("Day 2:");
    timed(solve_part1, 10);
    timed(solve_part2, 10);
    timed(solve_part1_stupid, 10);
    timed(solve_part2_stupid, 10);
}
pub fn solve_part1() -> impl Fn() {
    let games = include_str!("res/day2/games.txt").trim(); 
    let legal = Simd::<u8, 4>::from_array([12, 13, 14, 0]);
    let mut sum = 0;
    for (idx, game) in games.split('\n').enumerate() {
        let mut game_legal = true;
        let game = game.split(':').skip(1).next().unwrap();
        for group in game.split(';') {
            let mut colors = [0; 4];
            for color in group.split(',') {
                let mut split = color.trim().split(' ');
                let count = split.next().unwrap().parse::<u8>().unwrap();
                let color = split.next().unwrap().as_bytes()[0];
                let i = if color == b'r' {
                    0
                } else if color == b'g' {
                    1
                } else if color == b'b' {
                    2
                } else {panic!()};
                colors[i] = count; 
            }
            if legal.simd_lt(Simd::from_array(colors)).any() {
                game_legal = false;
                break;
            }
        }
        if game_legal {
            sum += idx + 1;
        }
    }
    move ||println!("\tPart 1: the sum of valid game ids is {0}", sum)
}
pub fn solve_part2() -> impl Fn() {
    let games = include_str!("res/day2/games.txt").trim(); 
    let mut sum = 0;
    for game in games.split('\n') {
        let game = game.split(':').skip(1).next().unwrap();
        let mut min_colors = Simd::from_array([0, 0, 0, 1]);
        for group in game.split(';') {
            let mut colors = [0, 0, 0, 1];
            for color in group.split(',') {
                let mut split = color.trim().split(' ');
                let count = split.next().unwrap().parse::<u8>().unwrap();
                let color = split.next().unwrap().as_bytes()[0];
                let i = if color == b'r' {
                    0
                } else if color == b'g' {
                    1
                } else if color == b'b' {
                    2
                } else {panic!()};
                colors[i] = count as u32; 
            }
            min_colors = min_colors.simd_max(Simd::from_array(colors));
        }
        sum += min_colors.reduce_product(); 
    }
    move ||println!("\tPart 2: the sum of minimum powers is {0}", sum)
}
pub fn solve_part1_stupid() -> impl Fn() {
    let games = include_str!("res/day2/games.txt").trim(); 
    // bgr
    let legal = Simd::<u8, 4>::from_array([14, 13, 12, 0]);
    let mut sum = 0;
    for (idx, game) in games.split('\n').enumerate() {
        let mut game_legal = true;
        let game = game.split(':').skip(1).next().unwrap();
        for group in game.split(';') {
            let mut colors = [0; 4];
            for color in group.split(',') {
                let mut split = color.trim().split(' ');
                let count = split.next().unwrap().parse::<u8>().unwrap();
                let color = split.next().unwrap().as_bytes()[0];
                let i = (color - 96) / 7;
                colors[i as usize] = count; 
            }
            if legal.simd_lt(Simd::from_array(colors)).any() {
                game_legal = false;
                break;
            }
        }
        if game_legal {
            sum += idx + 1;
        }
    }
    move ||println!("\tPart 1 stupid gimmick version: the sum of valid game ids is {0}", sum)
}
pub fn solve_part2_stupid() -> impl Fn() {
    let games = include_str!("res/day2/games.txt").trim(); 
    let mut sum = 0;
    for game in games.split('\n') {
        let game = game.split(':').skip(1).next().unwrap();
        let mut min_colors = Simd::from_array([0, 0, 0, 1]);
        for group in game.split(';') {
            let mut colors = [0, 0, 0, 1];
            for color in group.split(',') {
                let mut split = color.trim().split(' ');
                let count = split.next().unwrap().parse::<u8>().unwrap();
                let color = split.next().unwrap().as_bytes()[0];
                let i = (color - 96) / 7;
                colors[i as usize] = count as u32; 
            }
            min_colors = min_colors.simd_max(Simd::from_array(colors));
        }
        sum += min_colors.reduce_product(); 
    }
    move ||println!("\tPart 2 stupid gimmick version: the sum of minimum powers is {0}", sum)
}
