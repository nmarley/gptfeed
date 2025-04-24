use clap::Parser;
use std::collections::HashSet;
use std::fs;
use std::io::{self, Read, Write};

/// gptfeed - Output files in a specific format for LLM consumption
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input files to process. Use "-" to read from stdin
    files: Vec<String>,

    /// Container tag to use (defaults to "code")
    #[arg(short, long, default_value = "code")]
    container: String,

    /// Custom comment prefix to use (defaults to auto-detect based on file extension)
    #[arg(short = 'm', long, default_value = None, allow_hyphen_values = true)]
    comment_prefix: Option<String>,
}

fn main() {
    let args = Args::parse();
    if let Err(e) = process_input(
        &args.files,
        &args.container,
        args.comment_prefix.as_deref(),
        &mut io::stdout(),
    ) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn process_input<W: Write>(
    files: &[String],
    container: &str,
    comment_prefix: Option<&str>,
    writer: &mut W,
) -> io::Result<()> {
    let mut used: HashSet<String> = HashSet::new();
    let mut all_inputs = Vec::new();

    // Check if stdin should be used
    if files.is_empty() || files.contains(&"-".to_string()) {
        // Read from stdin
        let mut stdin_content = String::new();
        io::stdin().read_to_string(&mut stdin_content)?;

        // Ensure content ends with a newline for consistency
        if !stdin_content.is_empty() && !stdin_content.ends_with('\n') {
            stdin_content.push('\n');
        }

        // this is the "filename" for stdin, let's leave it empty
        all_inputs.push(("".to_string(), stdin_content));
    }

    // Add file contents
    for filename in files {
        // Skip stdin marker as we've already processed it
        if filename == "-" {
            continue;
        }

        // Skip if already processed
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

        all_inputs.push((filename.clone(), file_contents));
        used.insert(filename.clone());
    }

    // Write all content
    writeln!(writer, "<{}>", container)?;
    for (index, (filename, content)) in all_inputs.iter().enumerate() {
        let comment_string = if let Some(custom_comment) = comment_prefix {
            custom_comment
        } else if filename == "stdin" {
            "//"
        } else {
            let suffix = get_filetype_suffix(filename);
            match suffix.as_str() {
                "py" | "rb" => "#",
                "sql" => "--",
                _ => "//",
            }
        };

        writeln!(writer, "{} {}", comment_string, filename)?;
        write!(writer, "{}", content)?;

        // Print a newline if not the last input
        if index != all_inputs.len() - 1 {
            writeln!(writer)?;
        }
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
        let result = process_input(&files, container, None, &mut output);

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
        let result = process_input(&files, container, None, &mut output);

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
        let result = process_input(&files, container, comment_prefix, &mut output);

        // Check the result
        assert!(result.is_ok());

        // Convert output to string for examination
        let output_data = String::from_utf8(output.into_inner()).unwrap();

        // Exact expected output with full path
        let expected_output = format!("<code>\n; {}\ntest content\n</code>\n", temp_path);

        // Byte for byte comparison
        assert_eq!(output_data, expected_output);
    }

    // Note: We can't easily test stdin functionality in unit tests
    // as it would require mocking stdin. In real usage, the stdin
    // functionality would be tested manually or with integration tests.
}
