use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    input: String,

    #[arg(long)]
    part2: bool,

    #[arg(long)]
    debug: bool,
}

type PieceMask = [u8; 4];
const HLINE_MASK: [u8; 4] = [0b0011110, 0, 0, 0];
const PLUS_MASK: [u8; 4] = [0b0001000, 0b0011100, 0b0001000, 0];
const ELBOW_MASK: [u8; 4] = [0b0011100, 0b0000100, 0b0000100, 0];
const VLINE_MASK: [u8; 4] = [0b0010000, 0b0010000, 0b0010000, 0b0010000];
const SQUARE_MASK: [u8; 4] = [0b0011000, 0b0011000, 0, 0];

struct Board {
    // Rows of the board, with the bottom being the floor.
    // Each row is a bitmask of whether that column is solid.
    rows: Vec<u8>,

    // The height of the bottom of the board.
    floor: usize,

    // Whether there is currently a piece.
    has_piece: bool,

    // The bottom row of the piece and up.
    piece: [u8; 4],

    // The row that piece[0] is on, relative to floor.
    piece_y: usize,
}

impl Board {
    fn new() -> Self {
        Board {
            rows: Vec::new(),
            floor: 0,
            has_piece: false,
            piece: [0, 0, 0, 0],
            piece_y: 0,
        }
    }

    fn print(&self) {
        let mut top = self.rows.len();
        if self.has_piece {
            top = top.max(self.piece_y + 4);
        }
        for i in (0..top).rev() {
            print!("{:4} |", i + self.floor);
            let row = if i < self.rows.len() { self.rows[i] } else { 0 };
            let piece = if i >= self.piece_y && i < self.piece_y + 4 {
                self.piece[i - self.piece_y]
            } else {
                0
            };

            let mut mask = 0b01000000;
            for _ in 0..7 {
                if piece & mask != 0 {
                    print!("@");
                } else if row & mask != 0 {
                    print!("#");
                } else {
                    print!(".");
                }
                mask = mask >> 1;
            }

            println!("|");
        }
        println!("     +-------+");
    }

    fn move_right(&mut self) -> bool {
        if !self.has_piece {
            return false;
        }
        for i in 0..4 {
            if self.piece[i] & 0b0000001 != 0 {
                return false;
            }
            let new_piece = self.piece[i] >> 1;
            if i + self.piece_y < self.rows.len() {
                if self.rows[i + self.piece_y] & new_piece != 0 {
                    return false;
                }
            }
        }
        for i in 0..4 {
            self.piece[i] = self.piece[i] >> 1;
        }
        true
    }

    fn move_left(&mut self) -> bool {
        if !self.has_piece {
            return false;
        }
        for i in 0..4 {
            if self.piece[i] & 0b01000000 != 0 {
                return false;
            }
            let new_piece = self.piece[i] << 1;
            if i + self.piece_y < self.rows.len() {
                if self.rows[i + self.piece_y] & new_piece != 0 {
                    return false;
                }
            }
        }
        for i in 0..4 {
            self.piece[i] = self.piece[i] << 1;
        }
        true
    }

    fn move_down(&mut self) -> bool {
        if !self.has_piece {
            return false;
        }
        if self.piece_y == 0 {
            return false;
        }
        for i in 0..4 {
            let new_y = (self.piece_y + i) - 1;
            if new_y < self.rows.len() {
                if self.rows[new_y] & self.piece[i] != 0 {
                    return false;
                }
            }
        }
        self.piece_y = self.piece_y - 1;
        true
    }

    fn commit(&mut self) {
        let mut new_floor = 0;
        for i in 0..4 {
            if self.piece[i] == 0 {
                continue;
            }
            let new_y = self.piece_y + i;
            while self.rows.len() <= new_y {
                self.rows.push(0);
            }
            self.rows[new_y] = self.rows[new_y] | self.piece[i];
            if self.rows[new_y] == 0b01111111 {
                new_floor = new_y + 1;
            }
        }
        self.has_piece = false;

        if new_floor > 0 {
            self.floor = self.floor + new_floor;
            let mut new_rows = Vec::new();
            for i in new_floor..self.rows.len() {
                new_rows.push(self.rows[i]);
            }
            self.rows = new_rows;
        }
    }

    fn has_piece(&self) -> bool {
        self.has_piece
    }

    fn height(&self) -> usize {
        self.floor + self.rows.len()
    }

    fn place(&mut self, spec: &'static PieceMask) {
        self.has_piece = true;
        self.piece_y = self.rows.len() + 3;
        for i in 0..4 {
            self.piece[i] = spec[i];
        }
    }
}

fn read_input(path: &str, _debug: bool) -> Result<String> {
    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);
    loop {
        let mut line = String::new();
        let n = r.read_line(&mut line).unwrap();
        let line = line.trim();

        if line == "" {
            if n == 0 {
                break;
            }
            continue;
        }

        return Ok(line.to_string());
    }
    Err(anyhow!("no input!"))
}

fn simulate(input: &str, debug: bool) -> Result<usize> {
    let specs: Vec<&PieceMask> = vec![
        &HLINE_MASK,
        &PLUS_MASK,
        &ELBOW_MASK,
        &VLINE_MASK,
        &SQUARE_MASK,
    ];
    let mut spec_i = 0;

    // let winds = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    let winds = read_input(input, debug)?.chars().collect::<Vec<char>>();
    let mut wind_i = 0;

    let mut committed = 0u64;
    let mut board = Board::new();

    loop {
        if !board.has_piece() {
            let spec = specs[spec_i];
            spec_i = (spec_i + 1) % specs.len();

            board.place(spec);
            if debug {
                println!("New Piece (height = {})", board.height());
                board.print();
                println!("");
            }
        }

        let swind = winds[wind_i];
        wind_i = (wind_i + 1) % winds.len();
        if swind == '<' {
            board.move_left();
        } else {
            board.move_right();
        }

        // println!("Moved {}, {}", 1, 0);
        // board.print();
        // println!("");

        if !board.move_down() {
            board.commit();
            committed = committed + 1;
            if committed % 10000000 == 0 {
                println!(
                    "{} committed. height = {}, mem = {}",
                    committed,
                    board.height(),
                    board.rows.len()
                );
            }
            if debug && committed == 10 {
                return Ok(0);
            }
            if committed == 2022 {
                let ans = board.height();
                println!("ans = {}", ans);
            }
            if committed == 1000000000000 {
                let ans = board.height();
                println!("ans = {}", ans);
                return Ok(ans);
            }
        }
        // println!("Dropped");
        // board.print();
        // println!("");
    }
}

fn process(args: &Args) -> Result<()> {
    simulate(&args.input, args.debug)?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
