use std::fs;

use anyhow::{Error, Ok};

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// In the real world, it's common to use crates such as [`clap`] or
/// [`structopt`] to handle argument parsing, and of course doing so is
/// permitted in this exercise as well, though it may be somewhat overkill.
///
/// [`clap`]: https://crates.io/crates/clap
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
/// [`structopt`]: https://crates.io/crates/structopt
#[derive(Debug)]
pub struct Flags {
    line_number: bool, // -n
    reverse: bool,     // -v
    list_file: bool,   // -l
    match_icase: bool, // -i
    match_hole_line: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut s = Flags {
            line_number: false,
            reverse: false,
            list_file: false,
            match_icase: false,
            match_hole_line: false,
        };

        for &flag in flags.iter() {
            match flag {
                "-v" => s.reverse = true,
                "-n" => s.line_number = true,
                "-l" => s.list_file = true,
                "-i" => s.match_icase = true,
                "-x" => s.match_hole_line = true,
                _ => {}
            }
        }
        s
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut res = Vec::new();
    let with_filename = files.len() > 1;
    for &file_name in files {
        let content = fs::read_to_string(file_name)?;

        for (line_number, line) in content.lines().enumerate() {
            println!("Line{}: {}", line_number, line);
            let mut check_rs;
            if flags.match_icase {
                check_rs = line.to_lowercase().contains(&pattern.to_lowercase());
            } else {
                check_rs = line.contains(pattern);
            }

            if flags.match_hole_line {
                check_rs = check_rs && line.len() == pattern.len();
            }

            if flags.reverse {
                check_rs = !check_rs;
            }

            if check_rs {
                render_rs(file_name, line_number, line, &mut res, flags, with_filename);
            }
        }
    }

    Ok(res)
}

fn render_rs(
    file_name: &str,
    line_number: usize,
    line: &str,
    res: &mut Vec<String>,
    flags: &Flags,
    with_filename: bool,
) {
    if flags.list_file {
        let file_name = file_name.to_string();
        if !res.contains(&file_name) {
            res.push(file_name);
        }
    } else if flags.line_number {
        if with_filename {
            res.push(format!("{}:{}:{}", file_name, (line_number + 1), line));
        } else {
            res.push(format!("{}:{}", (line_number + 1), line));
        }
    } else {
        if with_filename {
            res.push(format!("{}:{}", file_name, line));
        } else {
            res.push(line.to_string());
        }
    }
}
