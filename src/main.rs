use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let mut used: HashSet<String> = HashSet::new();
    let files = env::args().skip(1).collect::<Vec<String>>();

    for f in files {
        let suffix = get_filetype_suffix(&f);
        let comment_string = match suffix.as_str() {
            "py" | "rb" => "#",
            "sql" => "--",
            _ => "//",
        };

        // skip if already printed
        if used.contains(&f) {
            continue;
        }
        let file_contents = fs::read_to_string(&f).unwrap();

        println!("{} {}", comment_string, f);
        println!("{}", file_contents);
        used.insert(f.clone());
    }
}

fn get_filetype_suffix(filename: impl Into<String>) -> String {
    let filename = filename.into();
    filename.split('.').last().unwrap_or("").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_filetype_suffix() {
        assert_eq!(get_filetype_suffix("test.py"), "py");
    }
}
