use std::{
  io::{self, BufRead, BufReader, Write},
  path::PathBuf,
};

use core::fmt;

// NOTE: you can give `main()` a return value which will just be `unwrap()`
// internally, which is very nice for testing out stuff
fn main() -> io::Result<()> {
  // NOTE: since `"history.txt".to_string()` is expensive, you only want to do
  // it if necessary, so we use `unwrap_or_else` which takes in a closure
  // instead of the value iteself
  let history_file_path = PathBuf::from(
    std::env::args()
      .nth(1)
      .unwrap_or_else(|| "history.txt".to_string()),
  );

  let history_file =
    std::fs::File::open(&history_file_path).map(BufReader::new)?;

  // NOTE: unfortunately `try_collect()` is yet stable, so i have to manually do
  // it using `try_fold()` which makes this look a little messier
  let history =
    history_file
      .lines()
      .try_fold(Vec::new(), |mut history, line| {
        history.push(line?);
        Ok::<_, io::Error>(history)
      })?;

  let mut stdout = io::stdout();
  let mut session = Session::new(history, 0);

  clear_and_position();
  println!("{}", session);
  prompt(&mut stdout)?;

  for line in io::stdin().lines() {
    let mut line = line?.trim().to_string();

    // commands
    if line.starts_with('/') {
      line.remove(0);
      match line.to_lowercase().as_str() {
        "e" | "exit" => break,
        "u" | "up" => session.up(1),
        "d" | "down" => session.down(1),
        _ => (),
      }
    } else {
      session.add(line);
    }

    clear_and_position();
    println!("{}", session);
    prompt(&mut stdout)?;
  }

  Ok(())
}

#[derive(Debug)]
struct Session {
  history: Vec<String>,
  offset: usize,
}

impl Session {
  #[inline]
  const fn new(history: Vec<String>, offset: usize) -> Self {
    Self { history, offset }
  }

  fn up(&mut self, amount: usize) {
    self.offset = self.offset.saturating_add(amount).min(self.history.len());
  }

  fn down(&mut self, amount: usize) {
    self.offset = self.offset.saturating_sub(amount).max(0);
  }

  fn add(&mut self, line: String) {
    self.history.push(line);
  }
}

impl fmt::Display for Session {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let (cols, rows) = termion::terminal_size().map_err(|_| fmt::Error)?;

    let empty_space = (rows as usize).saturating_sub(self.history.len() + 3);
    write!(f, "{}", "\n".repeat(empty_space))?;

    // NOTE: using a `try_for_each()` to catch errors is much nicer since it
    // cuts down on code
    self
      .history
      .iter()
      .enumerate()
      .rev()
      .skip(self.offset)
      .take(rows as usize - 3)
      .rev()
      .try_for_each(|(i, line)| writeln!(f, "{}: {}", i, line))?;

    writeln!(f, "{}", "─".repeat(cols as usize))?;
    write!(f, "OFFSET: {}", self.offset)
  }
}

fn clear_and_position() {
  println!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
}

fn prompt(stdout: &mut io::Stdout) -> io::Result<()> {
  stdout.write_all(b"> ")?;
  stdout.flush()
}
