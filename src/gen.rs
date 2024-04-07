use std::{
    cmp::min,
    fs::File,
    io::{BufRead, BufReader},
};

pub enum GenErr {
    FileAccess(String),
}

pub struct Gen {
    headers: Vec<(usize, String)>,
    min_cnt: usize,
}

impl Gen {
    pub fn parse(filename: &str) -> Result<Gen, GenErr> {
        let mut gen = Gen::default();

        let file = File::open(filename)
            .map_err(|_| GenErr::FileAccess(filename.to_string()))?;
        let reader = BufReader::new(file);

        let mut lines = reader.lines().filter_map(|l| l.ok()).into_iter();
        while let Some(line) = lines.next() {
            let Some(header) = Gen::get_header(line) else {
                continue;
            };
            gen.min_cnt = min(gen.min_cnt, header.0);
            gen.headers.push(header);
        }

        Ok(gen)
    }

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

    fn get_header(line: String) -> Option<(usize, String)> {
        let trim_line = line.trim();

        let res = trim_line.trim_start_matches('#');
        let cnt = trim_line.len() - res.len();
        if cnt == 0 || !res.starts_with(' ') {
            return None;
        }

        Some((cnt, res.trim().to_string()))
    }

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
}

impl Default for Gen {
    fn default() -> Self {
        Self {
            headers: Vec::new(),
            min_cnt: 6,
        }
    }
}
