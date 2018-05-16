use std::env;
use std::fs;

macro_rules! build_from_paths {
    ($base:expr, $($segment:expr),+) => {{
        let mut base: ::std::path::PathBuf = $base.into();
        $(
            base.push($segment);
        )*
        base
    }}
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // println!("Args: ");
    // for arg in &args {
    //     println!("{:?}", arg);
    // }

    let minimum_saves = args.get(1).unwrap().parse().unwrap();
    let from_folder = args.get(2).unwrap();
    let to_folder = args.get(3).unwrap();

    let mut files = Vec::new();
    for entry in fs::read_dir(from_folder)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            files.push(entry);
        }
    }

    files.sort_by(|a, b| {
        b.metadata()
            .unwrap()
            .modified()
            .unwrap()
            .cmp(&a.metadata().unwrap().modified().unwrap())
    });

    for index in minimum_saves..files.len() {
        let entry = files.get(index).unwrap();
        let to_file = build_from_paths!(to_folder, entry.file_name());
        println!("Moving {:?} to {:?}", entry.path(), &to_file);
        fs::rename(entry.path(), to_file)?;
    }

    Ok(())
}
