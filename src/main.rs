use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    let dir = env::args().nth(1).unwrap();
    // limit output to 150 lines
    let feed_limit: usize = 150;

    let mut buf: Vec<&str> = Vec::new();

    // TODO: write to files (filenames w/numbers) which I can feed in 1 at a time
    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
    {
        let file_contents = fs::read_to_string(entry.path()).unwrap();
        let file_lines: Vec<&str> = file_contents.split('\n').collect();
        // println!("{}", entry.path().display());
        if file_lines.len() <= feed_limit {
            // buf.push()
            print_entire_file(entry.path(), &file_lines, feed_limit);
        } else {
            print_as_parts(entry.path(), &file_lines, feed_limit);
        }
    }
}

fn print_as_parts<P: AsRef<Path>>(filename: P, lines: &[&str], feed_limit: usize) {
    let mut lines_printed: usize = 0;
    let mut count_parts_printed: usize = 0;

    while lines_printed < lines.len() {
        println!(
            "\nPart {} of file `{}`:",
            count_parts_printed + 1,
            filename.as_ref().display()
        );

        // print feed_limit lines at a time
        let start = feed_limit * count_parts_printed;
        // but consider if there are less than feed_limit lines left
        let end = core::cmp::min(lines.len(), feed_limit * (count_parts_printed + 1));
        println!("\n```{}```", lines[start..end].join("\n"));
        lines_printed += end - start;
        count_parts_printed += 1;
    }
    println!(
        "\nthat concludes the content of file `{}`.",
        filename.as_ref().display()
    );
}

fn print_entire_file<P: AsRef<Path>>(filename: P, lines: &[&str], _feed_limit: usize) {
    println!("\nContents of file `{}`:", filename.as_ref().display());
    println!("\n```{}```", lines.join("\n"));
}
