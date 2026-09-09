use std::cmp::min;

/// Struct for generating table of contents
#[derive(Debug)]
pub struct Gen {
    headers: Vec<(usize, String)>,
    min_cnt: usize,
    found: bool,
}

impl Gen {
    /// Parses the file for headers.
    ///
    /// **Behavior:**
    /// - If TOC marker is found, it only indexes headers after the marker.
    /// - If no marker is found, it indexes all headers.
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

    /// Generates table of contents from the indexed headers.
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
    ///
    /// **Behavior:**
    /// - If no token is found, it puts the TOC in the beginning of the file.
    /// - If HTML mdcon marker is found, it replaces that TOC inside it with
    ///   the updated one.
    pub fn insert_toc(&self, content: &str, toc: &str) -> String {
        let wrapped_toc =
            format!("<!-- mdcon-start -->\n{}<!-- mdcon-end -->\n", toc);

        if !self.found {
            return format!("{}{}", wrapped_toc, content);
        }

        let mut res = String::with_capacity(content.len() + wrapped_toc.len());
        let mut in_code = false;
        let mut in_toc = false;
        for line in content.lines() {
            let trim_line = line.trim();

            if !in_code && trim_line.starts_with("```") {
                in_code = true;
            } else if in_code && trim_line == "```" {
                in_code = false;
            }

            if !in_code {
                if Self::is_mdcon(line) {
                    res.push_str(&wrapped_toc);
                    continue;
                }

                if trim_line == "<!-- mdcon-start -->" {
                    in_toc = true;
                    res.push_str(&wrapped_toc);
                    continue;
                }

                if in_toc {
                    if trim_line == "<!-- mdcon-end -->" {
                        in_toc = false;
                    }
                    continue;
                }
            }

            res.push_str(&line);
            res.push('\n');
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

            if Self::is_mdcon(line) || trim_line == "<!-- mdcon-start -->" {
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
