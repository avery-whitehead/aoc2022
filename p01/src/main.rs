use std::{fs::File, io::Read};

struct ElfAcc {
    past: Vec<u64>,
    cur_elf: u64,
}

fn main() {
    let mut file = File::open("./src/input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read line");

    let elf_acc = contents.lines().fold(ElfAcc { past: Vec::new(), cur_elf: 0 }, |acc, x| {
        if let Ok(num) = x.parse::<u64>() {
            ElfAcc {
                past: acc.past,
                cur_elf: acc.cur_elf + num,
            }
        } else {
            ElfAcc {
                past: {
                    let mut new_v = acc.past;
                    new_v.push(acc.cur_elf);
                    new_v
                },
                cur_elf: 0,
            }
        }
    });

    let mut sorted = elf_acc.past.clone();
    sorted.sort();
    let top_3: u64 = sorted.as_slice()[sorted.len() - 3..].to_vec().iter().sum();
    println!("{}", top_3);
}