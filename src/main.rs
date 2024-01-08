use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let dir = env::args().nth(1).unwrap();
    // limit output to 150 lines
    let feed_limit: usize = 150;

    let mut buf: Vec<String> = Vec::new();

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
    {
        let file_contents = fs::read_to_string(entry.path()).unwrap();
        let file_lines: Vec<&str> = file_contents.split('\n').collect();
        if file_lines.len() <= feed_limit {
            for line in print_entire_file(entry.path(), &file_lines, feed_limit).into_iter() {
                buf.push(line);
            }
        } else {
            for line in print_as_parts(entry.path(), &file_lines, feed_limit).into_iter() {
                buf.push(line);
            }
        }
    }

    // now output is gathered, print out buf in batches of feed_limit lines
    for (outf_num, lines) in buf.chunks(feed_limit).enumerate() {
        let mut fh = File::create(format!("output_{}.md", outf_num)).unwrap();
        for line in lines {
            write!(fh, "{}", line).unwrap();
        }
        drop(fh);
    }
}

const TRIPLE_BACKTICK: &str = "```";

fn print_as_parts<P: AsRef<Path>>(filename: P, lines: &[&str], feed_limit: usize) -> Vec<String> {
    let mut buf: Vec<String> = Vec::new();
    let mut lines_printed: usize = 0;
    let mut count_parts_printed: usize = 0;

    while lines_printed < lines.len() {
        buf.push(format!(
            "\nPart {} of file `{}`:",
            count_parts_printed + 1,
            filename.as_ref().display()
        ));

        // print feed_limit lines at a time
        let start = feed_limit * count_parts_printed;
        // but consider if there are less than feed_limit lines left
        let end = core::cmp::min(lines.len(), feed_limit * (count_parts_printed + 1));
        buf.push(TRIPLE_BACKTICK.to_string());
        // buf.extend_from_slice(lines[start..end]);
        buf.extend(lines[start..end].iter().map(|s| s.to_string()));
        buf.push(TRIPLE_BACKTICK.to_string());
        lines_printed += end - start;
        count_parts_printed += 1;
    }
    buf.push(format!(
        "\nthat concludes the content of file `{}`.",
        filename.as_ref().display()
    ));
    buf
}

fn print_entire_file<P: AsRef<Path>>(
    filename: P,
    lines: &[&str],
    _feed_limit: usize,
) -> Vec<String> {
    let mut buf: Vec<String> = Vec::new();
    buf.push(format!(
        "\nContents of file `{}`:",
        filename.as_ref().display()
    ));

    buf.push(TRIPLE_BACKTICK.to_string());
    buf.extend(lines.iter().map(|s| s.to_string()));
    buf.push(TRIPLE_BACKTICK.to_string());

    buf
}
