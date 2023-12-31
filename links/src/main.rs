use std::convert::AsRef;
use std::io::Read;
use std::{env, fs, io};

/// Read the content of the given file as a single string.
fn read_file<S>(filepath: S) -> Result<String, io::Error>
where
    S: AsRef<str>,
{
    let mut file = fs::File::open(filepath.as_ref())?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

/// A single page.
#[derive(Debug)]
struct Page {
    title: String,
    url:   String,
}

impl std::fmt::Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let max_width =
            termsize::get().map(|size| size.cols).unwrap_or(u16::MAX) as usize;
        // `6` accounts for the total width of all formatting
        // characters.
        let width = self.title.len() + self.url.len() + 6;
        if width <= max_width {
            write!(f, " - {} ({})", self.title, self.url)
        } else {
            write!(f, " - {}\n   ({})", self.title, self.url)
        }
    }
}

#[derive(PartialEq)]
enum PageBuilder {
    Empty,
    Title { title: String, lineno: usize },
}

use PageBuilder::{Empty, Title};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: links FILE");
        std::process::exit(1);
    }

    let filepath = &args[1];
    let content = read_file(filepath)?;

    let mut build = Empty;
    let mut pages: Vec<Page> = vec![];

    for (idx, line) in content.lines().enumerate() {
        let line = line.to_owned();
        build = match build {
            Empty => Title {
                title:  line,
                lineno: idx + 1,
            },
            Title { title, .. } => {
                pages.push(Page { title, url: line });
                Empty
            },
        }
    }

    match build {
        Empty => {
            for page in pages {
                println!("{page}\n");
            }
        },
        Title { title, lineno } => {
            eprintln!(
                "invalid links file '{filepath}' in line {lineno}, no URL for \
                 title '{title}'"
            );
            std::process::exit(2);
        },
    }

    Ok(())
}
