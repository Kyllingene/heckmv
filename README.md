# heckmv

Converts case while renaming files and directories. Provides all the cases that
[heck](https://crates.io/crates/heck) does, while adding three: UPPERCASE,
lowercase (those two ignore word breaks), and Sentence case.

The case is selected via `-c/--case`. Note the following guidelines:
- Do not append `case` to your selection, e.g. `kebab` *not* `kebab-case`
- Casing and non-alphabetic characters are ignored, `Sen-TENCE` == `sentence`
- The shouty cases acknowledge the following forms:
  - `shouty_CASE`: `shoutysnake`
  - `shout_CASE` : `shoutsnake`
  - `upper_CASE`: `uppersnake`

Usage:
```
heckmv [options] <files...>
  -r/--recursive    : recurse into directories
  -d/--include-dirs : also rename directories
  -c/--case <case>  : specify the case (required)
  -h/--help         : show this help message

Renames files using the Rust `heck` crate for
most casing conversions. Available cases:
    UPPERCASE : ignores word breaks
    lowercase : ignores word breaks
    Title Case
    Sentence case

    UpperCamelCase
    lowerCamelCase
    snake_case
    kebab-case
    SHOUTY_SNAKE_CASE
    SHOUTY-KEBAB-CASE
    Train-Case
```
