use clap::Parser;
use std::collections::HashSet;
use std::fs;
use std::io::{self, Write};

/// gptfeed - Output files in a specific format for LLM consumption
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input files to process
    files: Vec<String>,

    /// Container tag to use (defaults to "code")
    #[arg(short, long, default_value = "code")]
    container: String,

    /// Custom comment prefix to use (defaults to auto-detect based on file extension)
    #[arg(short = 'm', long, default_value = None)]
    comment_prefix: Option<String>,
}

fn main() {
    let args = Args::parse();
    if let Err(e) = process_files(
        &args.files,
        &args.container,
        args.comment_prefix.as_deref(),
        &mut io::stdout(),
    ) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn process_files<W: Write>(
    files: &[String],
    container: &str,
    comment_prefix: Option<&str>,
    writer: &mut W,
) -> io::Result<()> {
    let mut used: HashSet<String> = HashSet::new();
    let count_files = files.len();

    writeln!(writer, "<{}>", container)?;
    for (index, filename) in files.iter().enumerate() {
        let comment_string = if let Some(custom_comment) = comment_prefix {
            custom_comment
        } else {
            let suffix = get_filetype_suffix(filename);
            match suffix.as_str() {
                "py" | "rb" => "#",
                "sql" => "--",
                _ => "//",
            }
        };

        // skip if already printed
        if used.contains(filename) {
            continue;
        }
        let file_contents = match fs::read(filename) {
            Ok(bytes) => {
                // Try multiple encodings, fall back to raw bytes if needed
                String::from_utf8(bytes.clone())
                    .unwrap_or_else(|_| String::from_utf8_lossy(&bytes).to_string())
            }
            Err(e) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Error reading file: {} – {}", filename, e),
                ));
            }
        };

        writeln!(writer, "{} {}", comment_string, filename)?;
        write!(writer, "{}", file_contents)?;

        // print a newline if not the last file
        if index != count_files - 1 {
            writeln!(writer)?;
        }
        used.insert(filename.clone());
    }
    writeln!(writer, "</{}>", container)?;
    Ok(())
}

fn get_filetype_suffix(filename: impl Into<String>) -> String {
    let filename = filename.into();
    filename.split('.').next_back().unwrap_or("").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::io::Write as IoWrite;
    use tempfile::NamedTempFile;

    #[test]
    fn test_get_filetype_suffix() {
        assert_eq!(get_filetype_suffix("test.py"), "py");
    }

    #[test]
    fn test_process_files_with_custom_container() {
        // Create a temporary test file
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "test content").unwrap();
        let temp_path = temp_file.path().to_string_lossy().to_string();

        let files = vec![temp_path.clone()];
        let container = "pre";
        let mut output = Cursor::new(Vec::new());

        // Process the files
        let result = process_files(&files, container, None, &mut output);

        // Check the result
        assert!(result.is_ok());

        // Convert output to string for examination
        let output_data = String::from_utf8(output.into_inner()).unwrap();

        // Exact expected output with full path
        let expected_output = format!("<pre>\n// {}\ntest content\n</pre>\n", temp_path);

        // Byte for byte comparison
        assert_eq!(output_data, expected_output);
    }

    #[test]
    fn test_process_files_with_default_container() {
        // Create a temporary test file
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "test content").unwrap();
        let temp_path = temp_file.path().to_string_lossy().to_string();

        let files = vec![temp_path.clone()];
        let container = "code"; // Default container
        let mut output = Cursor::new(Vec::new());

        // Process the files
        let result = process_files(&files, container, None, &mut output);

        // Check the result
        assert!(result.is_ok());

        // Convert output to string for examination
        let output_data = String::from_utf8(output.into_inner()).unwrap();

        // Exact expected output with full path
        let expected_output = format!("<code>\n// {}\ntest content\n</code>\n", temp_path);

        // Byte for byte comparison
        assert_eq!(output_data, expected_output);
    }

    #[test]
    fn test_process_files_with_custom_comment() {
        // Create a temporary test file
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "test content").unwrap();
        let temp_path = temp_file.path().to_string_lossy().to_string();

        let files = vec![temp_path.clone()];
        let container = "code";
        let comment_prefix = Some(";");
        let mut output = Cursor::new(Vec::new());

        // Process the files
        let result = process_files(&files, container, comment_prefix, &mut output);

        // Check the result
        assert!(result.is_ok());

        // Convert output to string for examination
        let output_data = String::from_utf8(output.into_inner()).unwrap();

        // Exact expected output with full path
        let expected_output = format!("<code>\n; {}\ntest content\n</code>\n", temp_path);

        // Byte for byte comparison
        assert_eq!(output_data, expected_output);
    }
}
