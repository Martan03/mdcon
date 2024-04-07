use std::{
    cmp::min,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{args::Args, err::gen_err::GenErr};

pub struct Gen {
    title: bool,
    headers: Vec<(usize, String)>,
    min_cnt: usize,
}

impl Gen {
    /// Parses given file
    pub fn parse(args: &Args, filename: &str) -> Result<Gen, GenErr> {
        let mut gen = Gen::default();
        if !args.skip_title {
            gen.title = true;
        }

        let file = File::open(filename)
            .map_err(|_| GenErr::FileAccess(filename.to_string()))?;
        let reader = BufReader::new(file);

        let mut lines = reader.lines().filter_map(|l| l.ok()).into_iter();
        while let Some(line) = lines.next() {
            let trim_line = line.trim();
            if trim_line.starts_with("```") {
                Gen::skip_code(&mut lines);
            }
            
            let Some(header) = Gen::get_header(trim_line) else {
                if !trim_line.is_empty() {
                    gen.title = true;
                }
                continue;
            };

            if header.0 == 1 && !gen.title {
                gen.title = true;
                continue;
            }

            gen.min_cnt = min(gen.min_cnt, header.0);
            gen.headers.push(header);
        }

        Ok(gen)
    }

    /// Generates contents
    pub fn gen(&self) -> String {
        let mut res = String::new();
        for (cnt, header) in self.headers.iter() {
            let offset = "    ".repeat(cnt - self.min_cnt);
            res.push_str(&format!(
                "{}- [{}](#{})\n",
                offset,
                header,
                Gen::get_header_id(header)
            ));
        }
        res
    }

    /// Gets header info from given line, None when not header
    fn get_header(line: &str) -> Option<(usize, String)> {
        let res = line.trim_start_matches('#');
        let cnt = line.len() - res.len();
        if cnt == 0 || !res.starts_with(' ') {
            return None;
        }

        Some((cnt, res.trim().to_string()))
    }

    /// Converts header text to header ID
    fn get_header_id(text: &str) -> String {
        let mut res = String::new();
        for c in text.to_lowercase().chars() {
            if c == ' ' {
                res.push('-');
            } else if c.is_alphanumeric() || c == '-' {
                res.push(c)
            }
        }
        res
    }

    fn skip_code<T>(lines: &mut T)
    where
        T: Iterator<Item = String>,
    {
        while let Some(line) = lines.next() {
            if line.trim() == "```" {
                break;
            }
        }
    }
}

impl Default for Gen {
    fn default() -> Self {
        Self {
            title: false,
            headers: Vec::new(),
            min_cnt: 6,
        }
    }
}
