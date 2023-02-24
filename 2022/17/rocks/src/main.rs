use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    input: String,

    #[arg(long)]
    count: u64,

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

    // Returns true if the commit reset the board state with a new floor.
    fn commit(&mut self) -> bool {
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
            true
        } else {
            false
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

fn simulate(input: &str, count: u64, debug: bool) -> Result<usize> {
    let specs: Vec<&PieceMask> = vec![
        &HLINE_MASK,
        &PLUS_MASK,
        &ELBOW_MASK,
        &VLINE_MASK,
        &SQUARE_MASK,
    ];
    let mut spec_i = 0;

    let winds = read_input(input, debug)?.chars().collect::<Vec<char>>();
    let mut wind_i = 0;

    let mut committed = 0u64;
    let mut board = Board::new();

    let mut reset_cache = HashMap::new();

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
            committed = committed + 1;
            if board.commit() {
                if let Some((prev_c, prev_h)) = reset_cache.get(&(spec_i, wind_i)) {
                    println!(
                        "Reset at type={}, wind={}. Was {} @ {}. Now {} @ {}",
                        spec_i,
                        wind_i,
                        prev_c,
                        prev_h,
                        committed,
                        board.height()
                    );

                    let delta_c = committed - prev_c;
                    let delta_h = board.height() - prev_h;
                    println!("Δc={}, Δh={}", delta_c, delta_h);

                    let remaining_c = count - committed;
                    println!("remaining = {}", remaining_c);

                    let cycles = remaining_c / delta_c;
                    if cycles == 0 {
                        println!("Not time warping. The end is near.");
                    } else {
                        committed = committed + cycles * delta_c;
                        board.floor = board.floor + (cycles as usize) * delta_h;

                        println!(
                            "Time warping by {} cycles to {} @ {}",
                            cycles,
                            board.height(),
                            committed
                        );
                        println!("");
                    }
                } else {
                    reset_cache.insert((spec_i, wind_i), (committed, board.height()));
                }
            }
            if committed % 100 == 0 {
                println!(
                    "{} committed. height = {}, mem = {}",
                    committed,
                    board.height(),
                    board.rows.len()
                );
            }
            if committed == count {
                return Ok(board.height());
            }
        }
        // println!("Dropped");
        // board.print();
        // println!("");
    }
}

fn process(args: &Args) -> Result<()> {
    let ans = simulate(&args.input, args.count, args.debug)?;
    println!("ans = {}", ans);
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
