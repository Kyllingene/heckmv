use errata::FallibleExt;
use heck::*;
use sarge::sarge;

use std::path::{Path, PathBuf};

sarge! {
    Args,

    #ok 'c' case: String,
    'd' include_dirs: bool,
    'r' recursive: bool,

    'h' help: bool,
}

#[derive(Debug, Clone, Copy)]
enum Case {
    Upper,
    Lower,

    Title,
    Sentence,

    UpperCamel,
    LowerCamel,

    Snake,
    ShoutySnake,

    Kebab,
    ShoutyKebab,

    Train,
}

macro_rules! shouty {
    ( $case:tt ) => {
        concat!("shout", $case) | concat!("shouty", $case) | concat!("upper", $case)
    };
}

impl Case {
    fn convert(&self, name: &str) -> String {
        match self {
            Case::Upper => name.to_uppercase(),
            Case::Lower => name.to_lowercase(),

            Case::Title => name.to_title_case(),
            Case::Sentence => {
                let name = Case::Title.convert(name);
                let first = name.chars().next().unwrap();
                [
                    first.to_uppercase().to_string(),
                    name[first.len_utf8()..].to_lowercase(),
                ]
                .join("")
            }

            Case::UpperCamel => name.to_upper_camel_case(),
            Case::LowerCamel => name.to_lower_camel_case(),

            Case::Snake => name.to_snake_case(),
            Case::ShoutySnake => name.to_shouty_snake_case(),

            Case::Kebab => name.to_kebab_case(),
            Case::ShoutyKebab => name.to_shouty_kebab_case(),

            Case::Train => name.to_train_case(),
        }
    }

    fn from_str(case: &str) -> Result<Self, &str> {
        let c = case
            .to_ascii_lowercase()
            .chars()
            .filter(|ch| ch.is_alphabetic())
            .collect::<String>();

        Ok(match c.as_str() {
            "upper" => Self::Upper,
            "lower" => Self::Lower,

            "title" => Self::Title,
            "sentence" => Self::Sentence,

            "camel" | "uppercamel" => Self::UpperCamel,
            "lowercamel" => Self::LowerCamel,

            "snake" => Self::Snake,
            shouty!("snake") => Self::ShoutySnake,

            "kebab" => Self::Kebab,
            shouty!("kebab") => Self::ShoutyKebab,

            "train" => Self::Train,

            _ => Err(case)?,
        })
    }
}

fn convert_all<S: AsRef<Path>>(files: &[S], case: Case, args: &Args) {
    for file in files {
        let file = file.as_ref();
        let path = PathBuf::from(file);

        if path.is_dir() {
            if args.recursive {
                let sub = std::fs::read_dir(&path)
                    .fail("failed to recurse into directory")
                    .map(|e| e.fail("failed to read directory").path())
                    .collect::<Vec<_>>();
                convert_all(&sub, case, args);
            }

            if !args.include_dirs {
                continue;
            }
        }

        let filename = path
            .file_stem()
            .fail("zero-length filenames not supported")
            .to_str()
            .fail("non-UTF8 strings not supported");
        let new_filename = case.convert(filename);

        let mut new_path = path.clone();
        new_path.set_file_name(new_filename);
        if let Some(ext) = path.extension() {
            new_path.set_extension(ext);
        }

        std::fs::rename(&path, new_path).fail(format!("failed to rename `{}`", path.display()));
    }
}

fn main() {
    let (args, files) = Args::parse().fail("failed to parse arguments");
    let files = &files[1..];

    if args.help {
        println!(include_str!("help.txt"));
        return;
    }

    let case = Case::from_str(args.case.as_ref().fail("must provide a case")).fail("invalid case");

    convert_all(files, case, &args);
}
