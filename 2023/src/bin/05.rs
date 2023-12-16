use advent::common::parse_all;
use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::option::Option;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(long)]
    debug: bool,
}

#[derive(Debug)]
struct Range {
    start: i64,
    len: i64,
}

impl Range {
    fn end(&self) -> i64 {
        self.start + self.len
    }
}

#[derive(Debug)]
struct MapRange {
    src: Range,
    dst_offset: i64,
}

impl MapRange {
    fn new(dst_start: i64, src_start: i64, len: i64) -> Self {
        MapRange {
            src: Range {
                start: src_start,
                len,
            },
            dst_offset: dst_start - src_start,
        }
    }

    fn from_str(line: &str) -> Result<Self> {
        let parts = parse_all(line.split_whitespace())?;
        if parts.len() != 3 {
            return Err(anyhow!("invalid line: {:?}", line));
        }

        Ok(MapRange::new(parts[0], parts[1], parts[2]))
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<MapRange>,
}

impl Map {
    fn map(&self, src: i64) -> i64 {
        let single = self.map_single(src);
        let range = self.map_range(&Range { start: src, len: 1 });
        if range.len() != 1 {
            panic!("returned range list does not have len=1");
        }
        if range[0].len != 1 {
            panic!("returned range does not have len=1");
        }
        if single != range[0].start {
            panic!("old answer {} != new answer {}", single, range[0].start);
        }
        single
    }

    fn map_single(&self, src: i64) -> i64 {
        match self.ranges.binary_search_by_key(&src, |r| r.src.start) {
            Ok(i) => src + self.ranges[i].dst_offset,
            Err(i) => {
                if i == 0 {
                    src
                } else if src >= self.ranges[i - 1].src.end() {
                    src
                } else {
                    src + self.ranges[i - 1].dst_offset
                }
            }
        }
    }

    fn map_range(&self, src: &Range) -> Vec<Range> {
        let mut v = Vec::new();
        let mut current: Range = Range {
            start: src.start,
            len: src.len,
        };
        while current.len > 0 {
            let (offset, len) = match self
                .ranges
                .binary_search_by_key(&current.start, |r| r.src.start)
            {
                Ok(i) => {
                    // The ranges start at the same point.
                    (
                        self.ranges[i].dst_offset,
                        min(current.len, self.ranges[i].src.len),
                    )
                }
                Err(i) => {
                    if i == 0 {
                        // The src range starts before the first map range.
                        if current.end() < self.ranges[i].src.start {
                            (0, current.len)
                        } else {
                            // Return the src up until the start of the first range.
                            (0, self.ranges[i].src.start - current.start)
                        }
                    } else if current.start >= self.ranges[i - 1].src.end() {
                        // This src starts after the previous mapping ends.
                        if i == self.ranges.len() {
                            // The src is after the whole mapping.
                            (0, current.len)
                        } else {
                            let next_start = self.ranges[i].src.start;
                            if next_start >= current.end() {
                                // The next range starts after this src ends.
                                (0, current.len)
                            } else {
                                // The next range starts before the current src ends.
                                (0, next_start - current.start)
                            }
                        }
                    } else {
                        // The current src starts during the previous range.
                        let range_len = self.ranges[i - 1].src.end() - current.start;
                        let len = min(range_len, current.len);
                        (self.ranges[i - 1].dst_offset, len)
                    }
                }
            };
            v.push(Range {
                start: current.start + offset,
                len: len,
            });
            current = Range {
                start: current.start + len,
                len: current.len - len,
            };
        }
        v
    }

    fn verify(&self) -> Result<()> {
        let mut previous = 0;
        for range in self.ranges.iter() {
            if range.src.start < previous {
                return Err(anyhow!(
                    "invalid range: {} < {} in {:?}",
                    range.src.start,
                    previous,
                    range
                ));
            }
            previous = range.src.end();
        }
        Ok(())
    }

    fn read(f: &mut BufReader<File>) -> Result<Self> {
        let mut map = Map { ranges: Vec::new() };
        loop {
            let mut line = String::new();
            let _n = f.read_line(&mut line).unwrap();
            let line = line.trim();

            if line == "" {
                break;
            }

            map.ranges.push(MapRange::from_str(line)?);
        }
        map.ranges.sort_by_key(|r| r.src.start);
        map.verify()?;
        Ok(map)
    }

    fn read_with_name(f: &mut BufReader<File>, name: &str) -> Result<Self> {
        let mut line = String::new();
        let n = f.read_line(&mut line).unwrap();
        let line = line.trim();

        if line == "" {
            if n == 0 {
                return Err(anyhow!("unexpected eof for map {:?}", name));
            }
            return Err(anyhow!("unexpected newline for map {:?}", name));
        }
        if line != name {
            return Err(anyhow!("expected {:?}, got {:?}", name, line));
        }

        Self::read(f)
    }
}

#[derive(Debug)]
struct Input {
    seeds: Vec<i64>,
    maps: Vec<Map>,
}

fn read_seeds(f: &mut BufReader<File>) -> Result<Vec<i64>> {
    let mut line = String::new();
    let n = f.read_line(&mut line).unwrap();
    let line = line.trim();

    if line == "" {
        if n == 0 {
            return Err(anyhow!("unexpected eof at start of file"));
        }
    }

    if !line.starts_with("seeds: ") {
        return Err(anyhow!("expected \"seeds: \", got {:?}", line));
    }
    let line = &line[7..];
    let seeds = parse_all(line.split_whitespace())?;

    let mut line = String::new();
    let _n = f.read_line(&mut line).unwrap();
    let line = line.trim();

    if line != "" {
        return Err(anyhow!("expected blank line after seeds. got {:?}", line));
    }

    Ok(seeds)
}

impl Input {
    fn location_for_seed(&self, seed: i64) -> i64 {
        let mut n = seed;
        for m in self.maps.iter() {
            n = m.map(n);
        }
        n
    }

    fn part1(&self) -> i64 {
        let mut loc = self.location_for_seed(self.seeds[0]);
        for seed in self.seeds.iter() {
            let new_loc = self.location_for_seed(*seed);
            if new_loc < loc {
                loc = new_loc;
            }
        }
        loc
    }

    fn part2(&self) -> i64 {
        let mut seeds = Vec::new();
        let mut start = 0;
        for (i, n) in self.seeds.iter().enumerate() {
            if i % 2 == 0 {
                start = *n;
            } else {
                let len = *n;
                seeds.push(Range { start, len });
            }
        }
        let mut current = seeds;
        for m in self.maps.iter() {
            let mut next = Vec::new();
            for range in current.iter() {
                let v = m.map_range(range);
                for next_range in v {
                    next.push(next_range);
                }
            }
            current = next;
        }
        let mut min_pos = current[0].start;
        for range in current.iter() {
            min_pos = min(min_pos, range.start);
        }
        min_pos
    }

    fn read(f: &mut BufReader<File>) -> Result<Self> {
        let mut input = Input {
            seeds: read_seeds(f)?,
            maps: Vec::new(),
        };
        input
            .maps
            .push(Map::read_with_name(f, "seed-to-soil map:")?);
        input
            .maps
            .push(Map::read_with_name(f, "soil-to-fertilizer map:")?);
        input
            .maps
            .push(Map::read_with_name(f, "fertilizer-to-water map:")?);
        input
            .maps
            .push(Map::read_with_name(f, "water-to-light map:")?);
        input
            .maps
            .push(Map::read_with_name(f, "light-to-temperature map:")?);
        input
            .maps
            .push(Map::read_with_name(f, "temperature-to-humidity map:")?);
        input
            .maps
            .push(Map::read_with_name(f, "humidity-to-location map:")?);
        Ok(input)
    }

    fn read_from_file(path: &str) -> Result<Self> {
        let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
        let mut r = BufReader::new(file);
        Self::read(&mut r)
    }
}

fn process(args: &Args) -> Result<()> {
    let input = Input::read_from_file(&args.input)?;
    println!("seed 79 -> {}", input.location_for_seed(79));
    println!("part 1: {}", input.part1());
    println!("part 2: {}", input.part2());
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
