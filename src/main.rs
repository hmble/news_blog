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
    let mut links: Vec<String> = Vec::new();
    let mut file_data = String::new();
    for (i, line) in content.lines().enumerate() {
        if line.starts_with("Number: ") {
            file_data = format!("{} ", line);
        }
        if line.starts_with("Date: ") {
            file_data = format!("{} {} \n\n", file_data, line);
        }
        if line.starts_with("## News") {
            start = i;
        }

        if line.starts_with("# Crate") {
            end = i;
        }
    }

    links.push(file_data.to_string());
    for (i, line) in content.lines().enumerate() {
        if i > start && i < end {
            if line.is_empty() {
                continue;
            }
            links.push(format!("{}\n", line));
        }
        if i == end {
            break;
        }
    }
    links.push("\n".to_string());

    for line in &links {
        write!(file, "{}", line)?;
    }

    Ok(())
}
