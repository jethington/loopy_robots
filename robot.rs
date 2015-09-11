// This program is written as a solution to an r/dailyprogrammer challenge:
//https://www.reddit.com/r/dailyprogrammer/comments/32vlg8/20150417_challenge_210_hard_loopy_robots/

#[derive(Copy, Clone, PartialEq)]
enum Direction {
  Up,
  Down,
  Left,
  Right
}

#[derive(Copy, Clone, PartialEq)]
struct Position {
  x: i32,
  y: i32,
}

fn turn_left(dir: Direction) -> Direction {
  match dir {
    Direction::Up => Direction::Left,
    Direction::Left => Direction::Down,
    Direction::Down => Direction::Right,
    Direction::Right => Direction::Up,
  }
}

fn turn_right(dir: Direction) -> Direction {
  match dir {
    Direction::Up => Direction::Right,
    Direction::Right => Direction::Down,
    Direction::Down => Direction::Left,
    Direction::Left => Direction::Up,
  }
}

fn go_straight(dir: Direction, pos: Position) -> Position {
  let mut result: Position = pos;
  match dir {
    Direction::Up => result.y += 1,
    Direction::Right => result.x += 1,
    Direction::Down => result.y -= 1,
    Direction::Left => result.x -= 1,
  }
  result
}

use std::io;
use std::io::Write;

fn main() {
  print!("Enter the command string:  ");
  io::stdout().flush().ok().expect("Could not flush stdout");
    
  let mut input: String = String::new();
  io::stdin().read_line(&mut input).ok().expect("Failed to read line");

  let start_position: Position = Position{ x: 0, y: 0};
  let mut pos: Position = start_position;
  let mut dir: Direction = Direction::Up;
  
  for x in 1..5 {
    for c in input.trim().chars() {
      match c {
        'L' => dir = turn_left(dir),
        'R' => dir = turn_right(dir),
        'S' => pos = go_straight(dir, pos),
         _  => println!("Ignored input: {}", c),
      }
    }
    if (dir == Direction::Up) && (pos == start_position) {
      println!("Loop found in {} cycles.", x);
      return;
    }
  }
  println!("No loop detected.");
}