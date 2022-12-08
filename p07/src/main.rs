const UPDATE_SPACE: u64 = 30000000;
const TOTAL_SPACE: u64 = 70000000;

struct Directory {
    file_size: u64,
}

fn new_dir() -> Directory {
    Directory { file_size: 0 }
}

fn change_dir(cur_dir: &mut Vec<Directory>, command: &str) -> Option<Directory> {
    let dir_name = command.split("cd").nth(1).map(str::trim);
    let mut dir = None;
    match dir_name {
        Some("/") => cur_dir.push(new_dir()),
        Some("..") => {
            dir = cur_dir.pop();
        }
        Some(_) => cur_dir.push(new_dir()),
        None => return None,
    }
    dir
}

fn add_size_to_dir(command: &str, dirs: &mut Vec<Directory>) -> Result<(), String> {
    let file_size = parse_file_size(command).ok_or("Couldn't get file size")?;
    for dir in dirs {
        dir.file_size += file_size;
    }
    Ok(())
}

fn parse_file_size(command: &str) -> Option<u64> {
    command.split(" ").nth(0)?.parse::<u64>().ok()
}

fn get_smallest_deletable_dir(dirs: Vec<Directory>, used_space: u64) -> u64 {
    let mut delete_fs = 0;
    let space_needed = UPDATE_SPACE - (TOTAL_SPACE - used_space);
    for dir in dirs {
        if dir.file_size >= space_needed && (dir.file_size < delete_fs || delete_fs == 0) {
            delete_fs = dir.file_size;
        }
    }
    delete_fs
}

fn get_total_space_used(input: &str) -> u64 {
    input.split("\n").fold(0, |acc, line| {
        if line.chars().next().map(char::is_numeric).unwrap_or(false) {
            acc + parse_file_size(line).unwrap_or(0)
        } else {
            acc
        }
    })
}

fn main() {
    let input = include_str!("./input.txt");
    let used_space = get_total_space_used(input);
    let mut dir_stack = Vec::new();
    let mut results = Vec::new();
    for command in input.split("\n") {
        if command.starts_with("$ cd") {
            let leaving_dir = change_dir(&mut dir_stack, command);
            if leaving_dir.is_some() {
                results.push(leaving_dir.unwrap())
            }
        } else if command.is_empty() {
            let final_dir = change_dir(&mut dir_stack, "$ cd ..");
            if final_dir.is_some() {
                results.push(final_dir.unwrap())
            }
        } else if !command.starts_with("dir") && !command.starts_with("$") {
            add_size_to_dir(command, &mut dir_stack);
        }
    }
    println!("{}", get_smallest_deletable_dir(results, used_space));
}
