# mdcon

Utility for generating **Table of contents** from Markdown file.

## Installation:
You have to compile it yourself, but that shouldn't be a problem. Only thing
you need is `cargo`:
```
cargo build -r
```
After its done compiling, you can start it in `./target/release/mdcon`

## Usage:
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

## Detailes description:
This utility generates **Table of contents** from Markdown file. Each item in
the table consists of the text, which is the text corresponding to the header
text and then the link itself, which redirects to corresponding header.

### Example:
If we have this Markdown:
```markdown
# mdcon test
## This is a title
bla bla bla

### Subtitle

## And so on...
bla bla bla
```

We get this as an output (note that by default it skips the title):
```markdown
- [This is a title](#this-is-a-title)
    - [Subtitle](#subtitle)
- [And so on...](#and-so-on)
```

## Technologies:
I used library called [termint](https://crates.io/crates/termint) for colored
printing.

## Links

- **Author:** [Martan03](https://github.com/Martan03)
- **GitHub repository:** [termint](https://github.com/Martan03/mdcon)
- **Author website:** [martan03.github.io](https://martan03.github.io)
