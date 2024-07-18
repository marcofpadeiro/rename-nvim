use std::error::Error;
use std::fs::File;
use std::fs::{self, rename};
use std::io::{stdin, BufRead, BufReader, Write};
use std::path::Path;
use std::process::{exit, Command};
use std::{env, io};

const TEMP_FILE_PATH: &str = "/tmp/file_names.txt";

struct FileRename {
    old: String,
    new: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 { &args[1] } else { "." };

    let mut files = get_folder_files(path).unwrap();
    write_into_file(&files);

    open_editor(TEMP_FILE_PATH);

    get_updated_names(TEMP_FILE_PATH, &mut files);

    files.iter().for_each(|file| {
        println!("{} -> {}", file.old, file.new);
    });

    get_confirmation();

    files.iter().for_each(|file| {
        rename(
            format!("{}/{}", path, &file.old),
            format!("{}/{}", path, &file.new),
        )
        .unwrap();
    });

    println!("Done! :)");
}

fn get_confirmation() {
    print!("\nDo you want to proceed? [Y/n]: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let input = input.trim();

    if input != "" && input != "y" && input != "Y" {
        return;
    }
}

fn get_folder_files(path: &str) -> Result<Vec<FileRename>, Box<dyn Error>> {
    let entries = fs::read_dir(Path::new(path))?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file())
        .collect::<Vec<_>>();

    let files = entries
        .iter()
        .map(|entry| FileRename {
            old: entry.file_name().into_string().unwrap(),
            new: String::new(),
        })
        .collect::<Vec<FileRename>>();
    Ok(files)
}

fn write_into_file(files: &Vec<FileRename>) {
    let mut output_file = File::create(TEMP_FILE_PATH).unwrap();

    for file in files {
        writeln!(output_file, "{}", file.old).unwrap();
    }
}

fn get_updated_names(path: &str, files: &mut Vec<FileRename>) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    if lines.len() != files.len() {
        eprintln!("Sizes don't match");
        exit(1);
    }

    files.iter_mut().zip(lines.iter()).for_each(|(x, y)| {
        x.new = y.clone();
    });
}

fn open_editor(file_path: &str) {
    let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
    Command::new(editor)
        .arg(file_path)
        .status()
        .expect("failed to open editor");
}
