use std::cmp::min;

/// Struct for generating table of contents
#[derive(Debug)]
pub struct Gen {
    headers: Vec<(usize, String)>,
    min_cnt: usize,
    found: bool,
}

impl Gen {
    /// Parses given file
    pub fn parse(content: &str) -> Gen {
        let mut gen = Gen::default();

        let mut lines = gen.locate_token(content);
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
        gen
    }

    /// Generates contents
    pub fn gen_toc(&self, max_ident: usize) -> String {
        let mut res = String::new();
        for (cnt, header) in self.headers.iter() {
            let ident = cnt - self.min_cnt;
            if ident >= max_ident {
                continue;
            }

            let offset = "    ".repeat(ident);
            res.push_str(&format!(
                "{}- [{}](#{})\n",
                offset,
                header,
                Gen::get_header_id(header)
            ));
        }
        res
    }

    /// Inserts the given table of contents into the content.
    pub fn insert_toc(&self, content: &str, toc: &str) -> String {
        if !self.found {
            return format!("{}{}", toc, content);
        }

        let mut res = String::with_capacity(content.len() + toc.len());
        let mut in_code = false;
        for line in content.lines() {
            let trim_line = line.trim();

            if !in_code && trim_line.starts_with("```") {
                in_code = true;
            } else if in_code && trim_line == "```" {
                in_code = false;
            }

            if !in_code && Self::is_mdcon(line) {
                res.push_str(toc);
            } else {
                res.push_str(&line);
                res.push('\n');
            }
        }
        res
    }

    /// Locates token in markdown
    fn locate_token<'a>(&mut self, content: &'a str) -> std::str::Lines<'a> {
        let mut lines = content.lines();
        while let Some(line) = lines.next() {
            let trim_line = line.trim();
            if trim_line.starts_with("```") {
                Gen::skip_code(&mut lines);
                continue;
            }

            if Self::is_mdcon(line) {
                self.found = true;
                return lines;
            }
        }

        self.found = false;
        content.lines()
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

    /// Skips code block in markdown
    fn skip_code<'a, T>(lines: &mut T)
    where
        T: Iterator<Item = &'a str>,
    {
        for line in lines {
            if line.trim() == "```" {
                break;
            }
        }
    }

    fn is_mdcon(line: &str) -> bool {
        line.chars().filter(|c| *c != ' ').eq("{{mdcon}}".chars())
    }
}

impl Default for Gen {
    fn default() -> Self {
        Self {
            headers: Vec::new(),
            min_cnt: 6,
            found: false,
        }
    }
}
