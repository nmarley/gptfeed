use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let mut used: HashSet<String> = HashSet::new();
    let files = env::args().skip(1).collect::<Vec<String>>();

    for f in files {
        // skip if already printed
        if used.contains(&f) {
            continue;
        }
        let file_contents = fs::read_to_string(&f).unwrap();

        println!("// {}", f);
        println!("{}", file_contents);
        used.insert(f.clone());
    }
}
