use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut args = env::args();
    args.next();

    for path in args {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open("links.txt")
            .unwrap();

        append_to_file(&path, &mut file).unwrap();
    }
}

pub fn append_to_file(
    path: &str,
    file: &mut fs::File,
) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let content = fs::read_to_string(path)?;

    let mut start: usize = 0;
    let mut end: usize = 0;
    for (i, line) in content.lines().enumerate() {
        if line.starts_with("## News") {
            start = i;
        }

        if line.starts_with("# Crate") {
            end = i;
        }
    }

    let mut links: Vec<&str> = Vec::new();
    for (i, line) in content.lines().enumerate() {
        if i > start && i < end {
            if line.is_empty() {
                continue;
            }
            links.push(line);
        }
        if i == end {
            break;
        }
    }

    for line in &links {
        write!(file, "{}\n", line)?;
    }

    Ok(())
}
