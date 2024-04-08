# mdcon

Utility for generating **Table of contents** from Markdown file.

## Table of Contents:
- [Installation](#installation)
- [Usage](#usage)
- [Detailes description](#detailes-description)
    - [Token to insert to file](#token-to-insert-to-file)
        - [Example](#example)
    - [Dumping Table of Contents](#dumping-table-of-contents)
- [Technologies](#technologies)
- [Links](#links)

## Installation
You have to compile it yourself, but that shouldn't be a problem. Only thing
you need is `cargo`:
```
cargo build -r
```
After its done compiling, you can start it in `./target/release/mdcon`

## Usage
Generates **Table of contents** for `README.md`:
```
./mdcon
```

Generates **Tables of contents** for given file:
```
./mdcon -f file.md
```

You can check other usage in help:
```
./mdcon -h
```

## Detailes description
This utility generates **Table of contents** from Markdown file. Each item in
the table consists of the text, which is the text corresponding to the header
text and then the link itself, which redirects to corresponding header.

### Token to insert to file
You can use special **token** to insert **Table of contens** to the file to
that location. If no token is found, **Table of contents** are placed to the
beginning of the file.

Token looks like this:
```
{{ mdcon }}
```
You can place it anywhere you want and `mdcon` will generate contents from
headers only after this token. After generating, it will automatically replace
this token with **Table of contents**.

#### Example
If we have this Markdown:
```markdown
# mdcon test
{{ mdcon }}

## This is a title
bla bla bla

### Sub title

## And so on...
bla bla bla
```

Our file will be modified to this:
```markdown
# mdcon test
- [This is a title](#this-is-a-title)
    - [Sub title](#sub-title)
- [And so on...](#and-so-on)

## This is a title
bla bla bla

### Sub title

## And so on...
bla bla bla
```

### Dumping Table of Contents
If you don't want to place **Table of contents** to the file, you can use `-d`
flag to dump the table to `stdout`.

## Technologies
I used library called [termint](https://crates.io/crates/termint) for colored
printing.

## Links

- **Author:** [Martan03](https://github.com/Martan03)
- **GitHub repository:** [termint](https://github.com/Martan03/mdcon)
- **Author website:** [martan03.github.io](https://martan03.github.io)
