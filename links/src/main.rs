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

enum PageBuilder {
    Empty,
    Title { title: String },
    Full { title: String, url: String },
}

use PageBuilder::{Empty, Full, Title};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: links FILE");
        std::process::exit(1);
    }

    let content = read_file(&args[1])?;
    let mut build = PageBuilder::Empty;
    let mut pages: Vec<Page> = vec![];

    for line in content.lines() {
        let line = line.to_owned();
        build = match build {
            Empty => Title { title: line },
            Title { title } => Full { title, url: line },
            Full { title, url } => {
                pages.push(Page { title, url });
                Title { title: line }
            },
        }
    }

    // One page must be left in the builder after the iterator
    // has finished.
    if let Full { title, url } = build {
        pages.push(Page { title, url });
    } else {
        println!("invalid links file");
        std::process::exit(2);
    }

    for page in pages {
        println!("{page}\n");
    }

    Ok(())
}
