use std::{
    cmp::min,
    fs::File,
    io::{BufRead, BufReader, Write},
};

use crate::err::gen_err::GenErr;

pub struct Gen {
    headers: Vec<(usize, String)>,
    min_cnt: usize,
}

impl Gen {
    /// Parses given file
    pub fn parse(filename: &str) -> Result<Gen, GenErr> {
        let mut gen = Gen::default();

        let file = File::open(filename)
            .map_err(|_| GenErr::FileAccess(filename.to_string()))?;
        let reader = BufReader::new(file);

        let mut lines =
            Gen::locate_token(reader.lines().filter_map(|l| l.ok()).collect());
        while let Some(line) = lines.next() {
            let trim_line = line.trim();
            if trim_line.starts_with("```") {
                Gen::skip_code(&mut lines);
            }

            let Some(header) = Gen::get_header(trim_line) else {
                continue;
            };

            gen.min_cnt = min(gen.min_cnt, header.0);
            gen.headers.push(header);
        }

        Ok(gen)
    }

    /// Generates contents
    pub fn gen(&self, filename: &str, dump: bool) {
        let mut res = String::new();
        for (cnt, header) in self.headers.iter() {
            let offset = "    ".repeat(cnt - self.min_cnt);
            res.push_str(&format!(
                "\n{}- [{}](#{})",
                offset,
                header,
                Gen::get_header_id(header)
            ));
        }
        if dump {
            print!("{res}");
        } else {
            _ = Gen::write_toc(filename, &res);
        }
    }

    /// Locates token in markdown
    fn locate_token(lines: Vec<String>) -> std::vec::IntoIter<String> {
        let mut lines_iter = lines.clone().into_iter();
        while let Some(line) = lines_iter.next() {
            let trim_line = line.replace(' ', "");
            if trim_line == "```" {
                Gen::skip_code(&mut lines_iter);
                continue;
            }
            if trim_line == "{{mdcon}}" {
                return lines_iter;
            }
        }
        lines.into_iter()
    }

    fn write_toc(filename: &str, toc: &str) -> Result<(), GenErr> {
        let file = File::open(filename)
            .map_err(|_| GenErr::FileAccess(filename.to_string()))?;
        let reader = BufReader::new(file);

        let mut res = String::new();
        for line in reader.lines() {
            let Ok(line) = line else {
                continue;
            };

            let trim_line = line.replace(' ', "");
            if trim_line == "{{mdcon}}" {
                res.push_str(toc);
            } else {
                res.push_str(&line);
            }
            res.push('\n');
        }

        let mut file = File::create(filename)
            .map_err(|_| GenErr::FileAccess(filename.to_string()))?;
        _ = file.write_all(res.as_bytes());
        Ok(())
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
            headers: Vec::new(),
            min_cnt: 6,
        }
    }
}
