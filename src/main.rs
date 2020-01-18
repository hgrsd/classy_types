use clap::{App, Arg};
use glob::glob;

mod classify;
mod io;
use classify::{ classify };
use io::{read_file, write_file};

pub fn traverse(paths: glob::Paths, out_path: Option<&str>) {
    for current_path in paths {
        if let Ok(path) = current_path {
            let file = read_file(&path);
            match file {
                Ok(file_content) => {
                    let classified_file = classify(&file_content);
                    if let Some(out) = out_path {
                        match write_file(&out, &path, &classified_file) {
                            Ok(path) => println!("Successfully written to {}", &path),
                            Err(_) => println!("Error writing to file."),
                        }
                    } else {
                        print!("{}", classified_file);
                    }
                }
                Err(e) => println!("Error reading {}: {}.", path.display(), e),
            }
        }
    }
}

fn main() {
    let matches = App::new("classy_types")
        .version("0.0.1")
        .author("Daniel Hogers <daniel.hogers@birdie.care>")
        .about("Generate TypeScript classes based on Types and Interfaces")
        .arg(
            Arg::with_name("out_path")
                .required(false)
                .short("o")
                .long("output")
                .value_name("PATH")
                .takes_value(true)
                .help("Specify output directory"),
        )
        .arg(
            Arg::with_name("pattern")
                .required(true)
                .help("File/directory pattern"),
        )
        .get_matches();

    match glob(matches.value_of("pattern").unwrap()) {
        Ok(paths) => {
            traverse(paths, matches.value_of("out_path"));
        }
        Err(_) => {
            println!("Invalid pattern.");
            return;
        }
    }
}
