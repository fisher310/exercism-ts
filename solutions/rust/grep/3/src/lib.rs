use std::{
    borrow::Cow,
    fs::File,
    io::{BufRead, BufReader},
};

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
#[derive(Debug, Default)]
pub struct Flags {
    line_number: bool, // -n
    reverse: bool,     // -v
    list_file: bool,   // -l
    match_icase: bool, // -i
    match_hole_line: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut obj_flag = Flags::default();
        for &f in flags {
            match f {
                "-v" => obj_flag.reverse = true,
                "-n" => obj_flag.line_number = true,
                "-l" => obj_flag.list_file = true,
                "-i" => obj_flag.match_icase = true,
                "-x" => obj_flag.match_hole_line = true,
                _ => {}
            }
        }
        obj_flag
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut res = Vec::new();

    let mut pattern = Cow::Borrowed(pattern);
    if flags.match_icase {
        *pattern.to_mut() = pattern.to_lowercase();
    }

    let pattern = match pattern {
        Cow::Owned(ref s) => s.as_str(),
        Cow::Borrowed(s) => s,
    };

    for &file_name in files {
        let file = BufReader::new(File::open(file_name)?);

        for (line_number, line) in file.lines().enumerate() {
            let line = line?;
            println!("Line{}: {}", line_number, line);
            let mut check_rs = {
                match (flags.match_hole_line, flags.match_icase) {
                    (true, true) => line.to_lowercase() == pattern,
                    (true, false) => line == pattern,
                    (false, true) => line.to_lowercase().contains(pattern),
                    (false, false) => line.contains(pattern),
                }
            };

            if flags.reverse {
                check_rs = !check_rs;
            }

            if check_rs {
                render_rs(
                    file_name,
                    line_number,
                    line,
                    &mut res,
                    flags,
                    files.len() > 1,
                );
                if flags.list_file {
                    break;
                }
            }
        }
    }

    Ok(res)
}

fn render_rs(
    file_name: &str,
    line_number: usize,
    line: String,
    res: &mut Vec<String>,
    flags: &Flags,
    with_filename: bool,
) {
    if flags.list_file {
        res.push(file_name.to_string());
    } else if flags.line_number {
        if with_filename {
            res.push(format!("{}:{}:{}", file_name, (line_number + 1), line));
        } else {
            res.push(format!("{}:{}", (line_number + 1), line));
        }
    } else if with_filename {
        res.push(format!("{}:{}", file_name, line));
    } else {
        res.push(line.to_string());
    }
}
