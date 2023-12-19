use advent::common::{read_lines, split_on, Array2D, StrIterator};
use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};
use std::{collections::VecDeque, str::FromStr, time::Duration};

#[derive(Debug)]
enum Direction {
    Up = 1,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            bail!("invalid direction: {}", s);
        }
        match s.chars().nth(0) {
            Some('U') => Ok(Direction::Up),
            Some('D') => Ok(Direction::Down),
            Some('L') => Ok(Direction::Left),
            Some('R') => Ok(Direction::Right),
            _ => Err(anyhow!("invalid direction: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct CellColor {
    r: u8,
    g: u8,
    b: u8,
}

impl FromStr for CellColor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 7 || s.chars().nth(0) != Some('#') {
            bail!("invalid color rgb hex: {}", s);
        }
        let r = u8::from_str_radix(&s[1..3], 16)?;
        let g = u8::from_str_radix(&s[3..5], 16)?;
        let b = u8::from_str_radix(&s[5..7], 16)?;
        Ok(CellColor { r, g, b })
    }
}

impl Into<Color> for CellColor {
    fn into(self) -> Color {
        Color::RGB(self.r, self.g, self.b)
    }
}

#[derive(Debug)]
struct Record {
    dir: Direction,
    amount: i64,
    color: CellColor,
}

impl FromStr for Record {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (dir, rest) = split_on(s, ' ').context("missing first space")?;
        let (amount, color) = split_on(rest, ' ').context("missing second space")?;
        if color.len() != 9 {
            bail!("invalid color part: {}", color)
        }
        let color = &color[1..8];

        let dir = dir.parse()?;
        let amount = amount.parse()?;
        let color = color.parse()?;

        Ok(Record { dir, amount, color })
    }
}

fn read_input(path: &str) -> Result<Vec<Record>> {
    Ok(read_lines(path)?.parse_all()?)
}

#[derive(Clone)]
enum Cell {
    Empty,
    Trench(CellColor),
    Filled,
}

fn create_grid(input: &Vec<Record>, debug: bool) -> Result<Array2D<Cell>> {
    let mut row = 0i64;
    let mut col = 0i64;
    let mut min_row = 0i64;
    let mut max_row = 0i64;
    let mut min_col = 0i64;
    let mut max_col = 0i64;
    for rec in input.iter() {
        match rec.dir {
            Direction::Up => row -= rec.amount,
            Direction::Down => row += rec.amount,
            Direction::Left => col -= rec.amount,
            Direction::Right => col += rec.amount,
        }
        min_row = min_row.min(row);
        max_row = max_row.max(row);
        min_col = min_col.min(col);
        max_col = max_col.max(col);
    }
    if debug {
        println!(
            "Position ranges from ({}, {}) to ({}, {})",
            min_row, min_col, max_row, max_col
        );
    }

    let rows = ((max_row - min_row) + 1) as usize;
    let cols = ((max_col - min_col) + 1) as usize;
    let grid = vec![vec![Cell::Empty; cols]; rows];
    let mut grid = Array2D::from_rows(grid)?;

    let mut row = 0i64;
    let mut col = 0i64;
    for rec in input.iter() {
        let (dr, dc) = match rec.dir {
            Direction::Up => (-rec.amount, 0),
            Direction::Down => (rec.amount, 0),
            Direction::Left => (0, -rec.amount),
            Direction::Right => (0, rec.amount),
        };

        let rstep = if dr < 0 { -1 } else { 1 };
        let cstep = if dc < 0 { -1 } else { 1 };
        let dr = dr.abs();
        let dc = dc.abs();

        for _ in 0..dr {
            row += rstep;
            let i = (row - min_row) as usize;
            let j = (col - min_col) as usize;
            grid[(i, j)] = Cell::Trench(rec.color.clone());
        }
        for _ in 0..dc {
            col += cstep;
            let i = (row - min_row) as usize;
            let j = (col - min_col) as usize;
            grid[(i, j)] = Cell::Trench(rec.color.clone());
        }
    }

    Ok(grid)
}

fn fill(grid: &mut Array2D<Cell>) {
    let mut q = VecDeque::new();
    for r in 0..grid.rows() {
        q.push_back((r, 0));
        q.push_back((r, grid.columns() - 1));
    }
    for c in 0..grid.columns() {
        q.push_back((0, c));
        q.push_back((grid.rows() - 1, c));
    }
    while let Some((r, c)) = q.pop_front() {
        if let Cell::Empty = grid[(r, c)] {
            if r > 0 {
                q.push_back((r - 1, c));
            }
            if c > 0 {
                q.push_back((r, c - 1));
            }
            if r < grid.rows() - 1 {
                q.push_back((r + 1, c));
            }
            if c < grid.columns() - 1 {
                q.push_back((r, c + 1));
            }
            grid[(r, c)] = Cell::Filled;
        }
    }
}

fn count_unfilled(grid: &Array2D<Cell>) -> u64 {
    let mut total = 0;
    for r in 0..grid.rows() {
        for c in 0..grid.columns() {
            match &grid[(r, c)] {
                Cell::Empty => total += 1,
                Cell::Trench(_) => total += 1,
                Cell::Filled => {}
            }
        }
    }
    total
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(long)]
    debug: bool,
}

fn display_grid(grid: &Array2D<Cell>) -> Result<()> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("AoC 2023 - Day 18", 1920, 1080)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_logical_size(grid.columns() as u32, grid.rows() as u32)?;
    canvas.set_draw_color(Color::RGB(40, 40, 40));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(40, 40, 40));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        for r in 0..grid.rows() {
            for c in 0..grid.columns() {
                let cell = &grid[(r, c)];
                let color = match cell {
                    Cell::Empty => Color::BLACK,
                    Cell::Filled => Color::RGB(20, 20, 20),
                    Cell::Trench(col) => col.clone().into(),
                };
                canvas.set_draw_color(color);
                canvas
                    .draw_rect(Rect::new(c as i32, r as i32, 1, 1))
                    .map_err(|s| anyhow!("{}", s))?;
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn process(args: &Args) -> Result<()> {
    let input = read_input(args.input.as_str())?;
    let mut grid = create_grid(&input, args.debug)?;
    fill(&mut grid);
    if args.debug {
        display_grid(&grid)?;
    }
    println!("ans: {}", count_unfilled(&grid));
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
