use advent::common::{read_lines, split_on, Array2D, StrIterator};
use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use itertools::Itertools;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};
use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
    time::Duration,
};

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

#[derive(Clone, Copy, Debug)]
struct VerticalSegment {
    column: i64,
    min_row: i64,
    max_row: i64,
}

#[derive(Clone, Copy, Debug)]
struct HorizontalSegment {
    row: i64,
    min_col: i64,
    max_col: i64,
}

fn create_segments(records: &Vec<Record>) -> (Vec<HorizontalSegment>, Vec<VerticalSegment>) {
    let mut h = Vec::new();
    let mut v = Vec::new();
    let mut r = 0i64;
    let mut c = 0i64;
    for rec in records {
        match rec.dir {
            Direction::Up => {
                v.push(VerticalSegment {
                    column: c,
                    min_row: r - rec.amount,
                    max_row: r,
                });
                r -= rec.amount;
            }
            Direction::Down => {
                v.push(VerticalSegment {
                    column: c,
                    min_row: r,
                    max_row: r + rec.amount,
                });
                r += rec.amount;
            }
            Direction::Left => {
                h.push(HorizontalSegment {
                    row: r,
                    min_col: c - rec.amount,
                    max_col: c,
                });
                c -= rec.amount;
            }
            Direction::Right => {
                h.push(HorizontalSegment {
                    row: r,
                    min_col: c,
                    max_col: c + rec.amount,
                });
                c += rec.amount;
            }
        };
    }
    (h, v)
}

/*
 * TODO: There's a bug here where:
 * 1) This gets counted as 6:
 *     ######
 *   > #    #
 *     ######
 * 2) This gets count as 6:
 *   > ######
 *     #    #
 *     ######
 * 3) But this gets counted as 6 + 6 = 12, not 11.
 *          ######
 *          #    #
 *     ######    #
 *     #         #
 *     ###########
 */

fn compute_area_for_row(
    row: i64,
    horizontal: &Vec<HorizontalSegment>,
    vertical: &Vec<VerticalSegment>,
    debug: bool,
) -> Result<i64> {
    if debug {
        println!("computing area for row {}", row);
    }

    // Find all the horizontal segments on this row.
    let horizontal = horizontal.iter().filter(|seg| seg.row == row).collect_vec();

    // Find all the vertical segments that cross this row.
    let mut vertical = vertical
        .iter()
        .filter(|seg| row >= seg.min_row && row <= seg.max_row)
        .collect_vec();

    // Sort all the segments by either min_col or col.
    let mut h_map = HashMap::new();
    for h in horizontal {
        h_map.insert(h.min_col, h.clone());
    }
    vertical.sort_by_key(|seg| seg.column);

    let mut total = 0i64;
    let mut inside = false;
    let mut top_edge = false;
    let mut trailing_edge: Option<HorizontalSegment> = None;
    let mut previous_column = i64::MIN;
    for v_seg in vertical {
        if debug {
            println!("considering vertical segment {:?}", v_seg);
        }
        let column = v_seg.column;

        if inside {
            // Add up the traversed area.
            if column <= previous_column {
                bail!(
                    "hit the same column twice {} vs {}.",
                    column,
                    previous_column
                );
            }
            let seg_len = column - previous_column + 1;
            if debug {
                println!("adding segment length {}", seg_len);
            }
            total += seg_len;
        }
        previous_column = column;

        if let Some(h_seg) = trailing_edge {
            // This better be the end of an edge.
            if debug {
                println!("this is a trailing edge");
            }
            if column != h_seg.max_col {
                bail!("edge does not end at corner.");
            }
            trailing_edge = None;

            if v_seg.min_row == row {
                // This is the top edge of the corner.
                if !top_edge {
                    inside = !inside;
                    // Don't double count the corner itself.
                    total -= 1;
                }
            } else if v_seg.max_row == row {
                // This is the bottom edge of the corner.
                if top_edge {
                    inside = !inside;
                    // Don't double count the corner itself.
                    total -= 1;
                }
            } else {
                bail!("found a t-junction.");
            }
        } else if let Some(h_seg) = h_map.get(&column) {
            // This is a leading edge.
            if debug {
                let seg_len = h_seg.max_col - h_seg.min_col + 1;
                println!("this is a leading edge. adding length {}", seg_len);
                total += seg_len;
            }
            trailing_edge = Some(h_seg.clone());
            if v_seg.min_row == row {
                top_edge = true;
            } else if v_seg.max_row == row {
                top_edge = false;
            } else {
                bail!("found a t-junction.");
            }
        } else {
            // This is not any kind of corner.
            if debug {
                println!("this is a not a corner");
            }
            inside = !inside;
        }
    }

    if inside {
        bail!("still inside at end of row {}", row);
    }

    Ok(total)
}

fn compute_area_by_segments(records: &Vec<Record>, debug: bool) -> Result<i64> {
    let mut total = 0;

    // Get all segments.
    let (mut horizontal, vertical) = create_segments(records);

    // Sort horizontal segments by row.
    horizontal.sort_by_key(|seg| seg.row);

    let mut previous_row = None;
    for segment in horizontal.iter() {
        if debug {
            println!("considering horizontal segment {:?}", segment);
        }
        let row = segment.row;

        if let Some(prev) = previous_row {
            if prev == row {
                continue;
            }
            if prev != row - 1 {
                // Compute the area for the row above this one.
                let row_area = compute_area_for_row(row - 1, &horizontal, &vertical, debug)?;

                // Multiply it by the height.
                let height = row - prev - 1;
                let area = row_area * height;
                if debug {
                    println!(
                        "add area for previous section with row {} * height {} = {}",
                        row_area, height, area
                    );
                }
                total += area;
            }
        }

        previous_row = Some(row);
        let row_area = compute_area_for_row(row, &horizontal, &vertical, debug)?;
        total += row_area;
    }

    Ok(total)
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
    let ans = count_unfilled(&grid);
    println!("ans (method 1): {}", ans);

    let ans = compute_area_by_segments(&input, args.debug)?;
    println!("ans (method 2): {}", ans);

    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
