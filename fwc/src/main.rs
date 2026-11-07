use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Default)]
struct Counts {
    lines: usize,
    bytes: usize,
    words: usize,
}

impl Counts {
    fn add(&mut self, other: &Counts) {
        self.lines += other.lines;
        self.bytes += other.bytes;
        self.words += other.words;
    }

    fn print(&self, name: &str, options: &Options) {
        let mut line = String::new();

        if options.lines {
            line.push_str(&format!("{:>8}", self.lines));
        }
        if options.words {
            line.push_str(&format!("{:>8}", self.words));
        }
        if options.bytes {
            line.push_str(&format!("{:>8}", self.bytes));
        }

        println!("{line} {name}");
    }
}

#[derive(Default)]
struct Options {
    lines: bool,
    words: bool,
    bytes: bool,
}

fn count(filename: &str) -> Result<Counts, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut buf = [0u8; 65536];
    let mut lines: usize = 0;
    let mut bytes: usize = 0;
    let mut words: usize = 0;
    let mut in_word = false;

    loop {
        let n = file.read(&mut buf)?;
        bytes += n;

        if n == 0 {
            break;
        }

        for &byte in &buf[..n] {
            if byte == b'\n' {
                lines += 1;
            }

            if byte.is_ascii_whitespace() {
                in_word = false;
            } else if !in_word {
                words += 1;
                in_word = true;
            }
        }
    }

    Ok(Counts {
        lines,
        bytes,
        words,
    })
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut files: Vec<&String> = Vec::new();
    let mut flags: Vec<&String> = Vec::new();
    let mut total: Counts = Counts::default();
    let mut failed = false;
    let mut options = Options::default();

    for arg in &args {
        if arg.starts_with("-") {
            flags.push(arg);
        } else {
            files.push(arg);
        }
    }

    if files.is_empty() {
        eprintln!("usage: fwc [-lwc] <file>");
        std::process::exit(1);
    };

    for flag in &flags {
        for c in flag[1..].chars() {
            match c {
                'l' => options.lines = true,
                'c' => options.bytes = true,
                'w' => options.words = true,
                _ => {
                    eprintln!("fwc: unknown option {c}");
                    std::process::exit(1);
                }
            }
        }
    }

    if !options.lines && !options.bytes && !options.words {
        options.lines = true;
        options.bytes = true;
        options.words = true;
    }

    let options = Options {
        lines: options.lines,
        bytes: options.bytes,
        words: options.words,
    };

    for file in &files {
        match count(file) {
            Ok(counts) => {
                counts.print(file, &options);
                total.add(&counts);
            }
            Err(error) => {
                eprintln!("{file}: {error}");
                failed = true;
            }
        }
    }

    if files.len() > 1 {
        total.print("total", &options);
    }

    if failed {
        std::process::exit(1);
    }
}
