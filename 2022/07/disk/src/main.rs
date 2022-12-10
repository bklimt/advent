use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,

    #[arg(long)]
    part2: bool,
}

struct KFile {
    _name: String,
    size: usize,
}

impl KFile {
    pub fn new(name: String, size: usize) -> KFile {
        KFile { _name: name, size }
    }
}

type KDirectoryRef<'a> = Rc<RefCell<KDirectory<'a>>>;

struct KDirectory<'a> {
    name: String,
    directories: HashMap<String, KDirectoryRef<'a>>,
    files: HashMap<String, KFile>,
    size: usize,
}

impl<'a> KDirectory<'a> {
    pub fn new(name: String) -> KDirectory<'a> {
        KDirectory {
            name: name,
            directories: HashMap::new(),
            files: HashMap::new(),
            size: 0,
        }
    }
}

fn print_cwd(cwd: &Vec<KDirectoryRef>) {
    for d in cwd.iter() {
        print!("{}/", d.borrow().name);
    }
    println!("");
}

fn process_cd(line: &str, cwd: &mut Vec<KDirectoryRef>) -> Result<()> {
    println!("cd: {:?}", line);
    if line == ".." {
        if cwd.len() == 1 {
            Err(anyhow!("cd above root!"))
        } else {
            cwd.pop();
            Ok(())
        }
    } else if line == "/" {
        while cwd.len() > 1 {
            cwd.pop();
        }
        Ok(())
    } else {
        let d = cwd.last().map(|r| r.clone());
        if let Some(d) = d {
            let dirs = &mut d.borrow_mut().directories;
            let d2: KDirectoryRef = match dirs.get_mut(line) {
                Some(d2) => d2.clone(),
                None => {
                    let d2 = Rc::new(RefCell::new(KDirectory::new(line.to_string())));
                    dirs.insert(line.to_string(), d2.clone());
                    d2
                }
            };
            cwd.push(d2);
            Ok(())
        } else {
            Err(anyhow!("root is missing somehow"))
        }
    }
}

fn process_ls(line: &str) -> Result<()> {
    println!("ls: {:?}", line);
    Ok(())
}

fn process_command(line: &str, cwd: &mut Vec<KDirectoryRef>) -> Result<()> {
    // println!("command: {:?}", line);
    match &line[0..2] {
        "cd" => {
            process_cd(&line[3..], cwd)?;
            print_cwd(cwd);
        }
        "ls" => process_ls(&line[2..])?,
        _ => {
            return Err(anyhow!("unknown command: {:?}", line));
        }
    };
    Ok(())
}

fn process_output(line: &str, cwd: &mut Vec<KDirectoryRef>) -> Result<()> {
    println!("output: {:?}", line);
    let space = line
        .find(' ')
        .ok_or_else(|| anyhow!("invalid output: {}", line))?;
    let (size_str, name) = line.split_at(space);
    if size_str == "dir" {
        return Ok(());
    }
    let name = &name[1..];
    let size = size_str
        .parse::<usize>()
        .with_context(|| format!("invalid output size: {:?}", line))?;
    println!("{}: {}", name, size);

    cwd.last()
        .unwrap()
        .borrow_mut()
        .files
        .insert(name.to_string(), KFile::new(name.to_string(), size));

    Ok(())
}

fn process_commands(path: &str) -> Result<KDirectoryRef> {
    let root = Rc::new(RefCell::new(KDirectory::new("".to_string())));
    let mut cwd: Vec<KDirectoryRef> = Vec::new();
    cwd.push(root.clone());

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

        // println!("{}", line);

        match line.chars().nth(0) {
            Some('$') => process_command(&line[2..], &mut cwd)?,
            Some(_) => process_output(line, &mut cwd)?,
            None => {
                return Err(anyhow!("empty line"));
            }
        };
    }
    Ok(root)
}

fn compute_size(root: KDirectoryRef) -> usize {
    let mut size = 0usize;
    for (_, f) in root.borrow().files.iter() {
        size = size + f.size;
    }
    for (_, d) in root.borrow().directories.iter() {
        size = size + compute_size(d.clone());
    }
    root.borrow_mut().size = size;
    size
}

fn sum_small(root: KDirectoryRef) -> usize {
    let mut size = 0usize;
    if root.borrow().size <= 100000 {
        size = size + root.borrow().size;
    }
    for (_, d) in root.borrow().directories.iter() {
        size = size + sum_small(d.clone());
    }
    size
}

fn process(path: &str, _part2: bool) -> Result<()> {
    let root = process_commands(path)?;
    compute_size(root.clone());
    let sum = sum_small(root.clone());
    println!("{}", sum);
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args.path, args.part2) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
