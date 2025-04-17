use clap::Parser;
use std::collections::HashSet;
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input files to process
    files: Vec<String>,

    /// Container tag to use (defaults to "code")
    #[arg(short, long, default_value = "code")]
    container: String,
}

fn main() {
    let args = Args::parse();
    let mut used: HashSet<String> = HashSet::new();
    let files = args.files.clone();
    let count_files = files.len();

    println!("<{}>", args.container);
    for (index, filename) in files.into_iter().enumerate() {
        let suffix = get_filetype_suffix(&filename);
        let comment_string = match suffix.as_str() {
            "py" | "rb" => "#",
            "sql" => "--",
            _ => "//",
        };

        // skip if already printed
        if used.contains(&filename) {
            continue;
        }
        let file_contents = match fs::read(&filename) {
            Ok(bytes) => {
                // Try multiple encodings, fall back to raw bytes if needed
                String::from_utf8(bytes.clone())
                    .unwrap_or_else(|_| String::from_utf8_lossy(&bytes).to_string())
            }
            Err(e) => {
                eprintln!("Error reading file: {} – {}", filename, e);
                continue;
            }
        };

        println!("{} {}", comment_string, filename);
        print!("{}", file_contents);

        // print a newline if not the last file
        if index != count_files - 1 {
            println!();
        }
        used.insert(filename.clone());
    }
    println!("</{}>", args.container);
}

fn get_filetype_suffix(filename: impl Into<String>) -> String {
    let filename = filename.into();
    filename.split('.').next_back().unwrap_or("").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_filetype_suffix() {
        assert_eq!(get_filetype_suffix("test.py"), "py");
    }
}
