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
    part2: bool,

    #[arg(long)]
    debug: bool,
}

type PieceSpec = [(i32, i32)];

const HLINE_SPEC: [(i32, i32); 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];
const PLUS_SPEC: [(i32, i32); 5] = [(1, -2), (0, -1), (1, -1), (2, -1), (1, 0)];
const ELBOW_SPEC: [(i32, i32); 5] = [(2, -2), (2, -1), (2, 0), (1, 0), (0, 0)];
const VLINE_SPEC: [(i32, i32); 4] = [(0, 0), (0, -1), (0, -2), (0, -3)];
const SQUARE_SPEC: [(i32, i32); 4] = [(0, -1), (1, -1), (0, 0), (1, 0)];

#[derive(Clone, Copy)]
enum TileType {
    Space,
    FixedRock,
    LooseRock,
}

struct Board {
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,

    tiles: HashMap<(i32, i32), TileType>,

    piece_spec: Option<&'static PieceSpec>,
    piece_x: i32,
    piece_y: i32,
}

impl Board {
    fn new() -> Self {
        Board {
            start_x: 0,
            start_y: 0,
            end_x: 7,
            end_y: 1,
            tiles: HashMap::new(),
            piece_spec: None,
            piece_x: 0,
            piece_y: 0,
        }
    }

    fn get(&self, x: i32, y: i32) -> TileType {
        *self.tiles.get(&(x, y)).unwrap_or(&TileType::Space)
    }

    fn set(&mut self, x: i32, y: i32, tile: TileType) {
        self.start_x = self.start_x.min(x);
        self.start_y = self.start_y.min(y);
        self.end_x = self.end_x.max(x + 1);
        self.end_y = self.end_y.max(y + 1);
        self.tiles.insert((x, y), tile);
    }

    fn print(&self) {
        for y in self.start_y..self.end_y {
            print!("{:4} |", y);
            for x in self.start_x..self.end_x {
                let mut typ = self.get(x, y);

                if let Some(spec) = self.piece_spec {
                    for coords in spec {
                        if x == self.piece_x + coords.0 && y == self.piece_y + coords.1 {
                            typ = TileType::LooseRock;
                        }
                    }
                }

                print!(
                    "{}",
                    match typ {
                        TileType::Space => '.',
                        TileType::FixedRock => '#',
                        TileType::LooseRock => '@',
                    }
                );
            }
            println!("|");
        }
        print!("     +");
        for _ in self.start_x..self.end_x {
            print!("-");
        }
        println!("+");
    }

    fn can_move(&self, dx: i32, dy: i32) -> bool {
        if let Some(spec) = self.piece_spec {
            for coords in spec {
                let p = (self.piece_x + dx + coords.0, self.piece_y + dy + coords.1);
                if p.0 < 0 || p.0 >= 7 {
                    return false;
                }
                if p.1 > 0 {
                    return false;
                }
                match self.get(p.0, p.1) {
                    TileType::Space => {}
                    TileType::FixedRock | TileType::LooseRock => {
                        return false;
                    }
                }
            }
            true
        } else {
            false
        }
    }

    fn move_piece(&mut self, dx: i32, dy: i32) -> bool {
        if !self.can_move(dx, dy) {
            false
        } else {
            self.piece_x = self.piece_x + dx;
            self.piece_y = self.piece_y + dy;

            if let Some(spec) = self.piece_spec {
                for coords in spec {
                    let p = (self.piece_x + coords.0, self.piece_y + coords.1);
                    // This is just to update min/max x/y.
                    self.set(p.0, p.1, TileType::Space);
                }
            }

            true
        }
    }

    fn commit(&mut self) {
        if let Some(spec) = self.piece_spec {
            for coords in spec {
                let p = (self.piece_x + coords.0, self.piece_y + coords.1);
                self.set(p.0, p.1, TileType::FixedRock);
            }
            self.piece_spec = None
        }
    }

    fn has_piece(&self) -> bool {
        self.piece_spec.is_some()
    }

    fn height(&self) -> i32 {
        for y in self.start_y..self.end_y {
            for x in self.start_x..self.end_x {
                if let TileType::FixedRock = self.get(x, y) {
                    return (y * -1) + 1;
                }
            }
        }
        0
    }

    fn place(&mut self, spec: &'static PieceSpec) -> bool {
        let mut min_y = -3;
        for y in self.start_y..self.end_y {
            if y - 4 >= min_y {
                break;
            }
            for x in self.start_x..self.end_x {
                if let TileType::FixedRock = self.get(x, y) {
                    min_y = y - 4;
                    break;
                }
            }
        }

        self.piece_spec = Some(spec);
        self.piece_x = 2;
        self.piece_y = min_y;
        for coords in spec {
            let p = (self.piece_x + coords.0, self.piece_y + coords.1);
            match self.get(p.0, p.1) {
                // The `set` call is just to update the min/max x/y.
                TileType::Space => self.set(p.0, p.1, TileType::Space),
                TileType::FixedRock | TileType::LooseRock => {
                    return false;
                }
            }
        }
        true
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

fn simulate(input: &str, debug: bool) -> Result<i32> {
    let specs: Vec<&PieceSpec> = vec![
        &HLINE_SPEC,
        &PLUS_SPEC,
        &ELBOW_SPEC,
        &VLINE_SPEC,
        &SQUARE_SPEC,
    ];
    let mut spec_i = 0;

    // let winds = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    let winds = read_input(input, debug)?;
    let mut wind_i = 0;

    let mut committed = 0;
    let mut board = Board::new();

    loop {
        if !board.has_piece() {
            let spec = specs[spec_i];
            spec_i = (spec_i + 1) % specs.len();

            board.place(spec);
            if debug {
                println!("New Piece");
                board.print();
                println!("");
            }
        }

        let swind = winds.chars().nth(wind_i).unwrap();
        wind_i = (wind_i + 1) % winds.len();
        let dx = if swind == '<' { -1 } else { 1 };

        board.move_piece(dx, 0);
        // println!("Moved {}, {}", 1, 0);
        // board.print();
        // println!("");

        if !board.move_piece(0, 1) {
            board.commit();
            committed = committed + 1;
            if committed == 2022 {
                let ans = board.height();
                println!("ans = {}", board.height());
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
