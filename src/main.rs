use clap::Parser;
use std::{
  cmp::max,
  fs,
  io::{self, Write},
};
use terminal_size::{terminal_size, Height, Width};

fn main() {
  let mut history: Vec<String> = Vec::new();

  if let Ok(contents) = fs::read_to_string("history.txt") {
    history = contents.split('\n').map(|s| s.to_string()).collect();
  }

  loop {
    clear();
    display(&history);
    prompt();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.as_str().trim() {
      "exit" => break,
      _ => (),
    }

    history.push(input.trim().to_string());

    fs::write("history.txt", history.join("\n")).unwrap();
  }
}

fn prompt() {
  print!("> ");
  io::stdout().flush().unwrap();
}

fn clear() {
  print!("\x1B[2J\x1B[1;1H");
}

fn display(history: &Vec<String>) {
  let width = terminal_size().map(|(Width(w), _)| w).unwrap_or(80) as usize;
  let height = terminal_size().map(|(_, Height(h))| h).unwrap_or(24) as usize;

  let empty_space =
    max(0, height.saturating_sub(2).saturating_sub(history.len()));

  for string in history.iter().rev().take(height - 2).rev() {
    println!("{}", string);
  }

  for _ in 0..empty_space {
    println!();
  }

  println!("{}", "─".repeat(width));
}
