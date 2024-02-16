use std::{
    error::Error,
    fmt::{self, Display},
    io::Write,
    vec,
};

use log::{debug, error, info};

use console::{style, Term};

use range::Range;

struct Game {
    size: usize,
    chess: Vec<Vec<u8>>,
    last_x: usize,
    last_y: usize,
}

impl Game {
    fn new(size: usize) -> Self {
        let mut chess = Vec::new();
        chess.resize(size, Vec::new());
        for line in chess.iter_mut() {
            line.resize(size, 0);
        }

        Game {
            size,
            chess,
            last_x: 0,
            last_y: 0,
        }
    }

    fn set_dot(&mut self, x: usize, y: usize, dot: u8) {
        self.last_x = x;
        self.last_y = y;
        self.chess[x][y] = dot;
    }

    fn check(&self) -> Option<u8> {
        let mut i: i32 = self.last_x as i32;
        let mut j: i32 = self.last_y as i32;
        let mut count = 0;
        let dot = self.chess[self.last_x][self.last_y];

        while i >= 0 && self.chess[i as usize][j as usize] == dot {
            i -= 1;
            count += 1;
        }
        i = (self.last_x + 1) as i32;
        while i < self.size as i32 && self.chess[i as usize][j as usize] == dot {
            i += 1;
            count += 1;
        }
        if count >= 5 {
            return Some(dot);
        }

        i = self.last_x as i32;
        count = 0;
        while j >= 0 && self.chess[i as usize][j as usize] == dot {
            j -= 1;
            count += 1;
        }
        j = (self.last_y + 1) as i32;
        while j < self.size as i32 && self.chess[i as usize][j as usize] == dot {
            j += 1;
            count += 1;
        }
        if count >= 5 {
            return Some(dot);
        }

        j = self.last_y as i32;
        count = 0;
        while j >= 0 && i >= 0 && self.chess[i as usize][j as usize] == dot {
            i -= 1;
            j -= 1;
            count += 1;
        }
        i = (self.last_x + 1) as i32;
        j = (self.last_y + 1) as i32;
        while j < self.size as i32
            && i < self.size as i32
            && self.chess[i as usize][j as usize] == dot
        {
            i += 1;
            j += 1;
            count += 1;
        }
        if count >= 5 {
            return Some(dot);
        }

        i = self.last_x as i32;
        j = self.last_y as i32;
        count = 0;
        while j >= 0 && i < self.size as i32 && self.chess[i as usize][j as usize] == dot {
            i += 1;
            j -= 1;
            count += 1;
        }
        i = (self.last_x + 1) as i32;
        j = (self.last_y + 1) as i32;
        while j >= 0 && i < self.size as i32 && self.chess[i as usize][j as usize] == dot {
            i -= 1;
            j += 1;
            count += 1;
        }
        if count >= 5 {
            return Some(dot);
        }

        None
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let encoder = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut x = String::new();

        x.push_str("  ");
        for i in Range::new(0, self.size).iter() {
            x.push_str(&format!("{} ", encoder[i] as char));
        }
        x.push_str("  \n");
        write!(f, "{}", x)?;

        let mut i = 0;
        for line in self.chess.iter() {
            write!(f, "{} ", encoder[i] as char)?;
            for dot in line.iter() {
                let ch = match dot {
                    1 => '*',
                    2 => 'x',
                    _ => ' ',
                };

                write!(f, "{} ", ch)?;
            }
            write!(f, " {}\n", encoder[i] as char)?;
            i += 1;
        }

        write!(f, "{}", x)?;

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut term = Term::stdout();
    let stdin = std::io::stdin();
    let mut size = String::new();

    term.clear_screen()?;
    term.write_line("Welcome to Mitsuha's chess game!\n")?;
    term.write_all(b"Please input chess size:")?;
    stdin
        .read_line(&mut size)
        .expect("error while reading the number!");
    let size: usize = size
        .trim()
        .parse()
        .expect("expect a number ranging from 16 to 62!");

    if size < 16 {
        panic!("size too small!");
    } else if size > 62 {
        panic!("size too large!");
    }

    let mut game = Game::new(size);

    term.clear_screen()?;
    term.write_line(&format!("\n\n{}", game))?;

    Ok(())
}
