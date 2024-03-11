use clap::{command, Arg, ArgMatches};
use std::env;
use std::fs;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;

fn main() {
    // let set_path_arg = Arg::new("setpath")
    //     .short('s')
    //     .long("set-path")
    //     .aliases(["stp", "setpath"])
    //     .help("Set the path to the directory where the files are created");

    // let create_arg = Arg::new("create")
    //     .short('c')
    //     .long("create")
    //     .aliases(["crt", "create"])
    //     .help("Create a new file");

    let match_result: ArgMatches = command!()
        .arg(
            Arg::new("setpath")
                .short('s')
                .long("set-path")
                .aliases(&["stp", "setpath"])
                .help("Set the path to the directory where the files are created")
                .takes_value(true),
        )
        .arg(
            Arg::new("create")
                .short('c')
                .long("create")
                .aliases(&["crt", "create"])
                .help("Create a new file")
                .takes_value(true),
        )
        .get_matches();

    if match_result.is_present("setpath") {
        set_path(match_result);
    } else if match_result.is_present("create") {
        create_file(match_result);
    }
}

fn set_path(match_result: clap::ArgMatches) {
    let path = match_result.value_of("setpath").unwrap();
    let env_path = Path::new(".env");
    let env_path = env_path.to_str().unwrap();

    let env_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(env_path);

    match env_file {
        Ok(file) => {
            let mut file = file;

            // Limpando o .env para escrever o novo path
            file.set_len(0).unwrap();
            file.write_all(format!("PATH={}", path).as_bytes()).unwrap();
            
            std::process::Command::new("clear").status().unwrap();
            println!("Path set to: {}", path);
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}

fn create_file(match_result: ArgMatches) {
    let file_name_input = match_result.value_of("create").unwrap();
    let file_name = file_name_input.replace("/", "_") + ".txt";

    // Lendo o arquivo .env
    let env_path = Path::new(".env");
    let env_path = env_path.to_str().unwrap();
    let env_file = fs::OpenOptions::new().read(true).open(env_path);

    let path: String = match env_file {
        Ok(file) => {
            let reader = std::io::BufReader::new(file);
            let mut path = String::new();

            for line in reader.lines() {
                path = line.unwrap();
            }

            path
        }
        Err(error) => {
            println!("Error: {}", error);
            String::from("")
        }
    };

    let path = path.replace("PATH=", "");
    let path = path.trim();

    let file_path = format!("{}/{}", path, file_name);
    let file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path);

    let file_content = format!("Apontamento - {:?}\n\n8h ->\n\n========================\nTODO's - 01/02/2024\n\n- [  ] <TODO>\n\n========================", file_name_input);

    match file {
        Ok(file) => {
            let mut file = file;
            file.write_all(b"").unwrap();
            file.write_all(file_content.as_bytes()).unwrap();
            
            std::process::Command::new("clear").status().unwrap();
            println!("File created: {}", file_name);
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }


}