fn main() {
    let input = include_str!("./input.txt");

    // Part 1
    let contains_count = input.split("\n").fold(0, |overlaps, pair| {
        let (elf_1, elf_2) = match pair.split_once(",") {
            Some(elves) => elves,
            None => return overlaps,
        };
        if has_sector_contains(elf_1, elf_2) {
            overlaps + 1
        } else {
            overlaps
        }
    });

    // Part 2
    let overlap_count = input.split("\n").fold(0, |overlaps, pair| {
        let (elf_1, elf_2) = match pair.split_once(",") {
            Some(elves) => elves,
            None => return overlaps,
        };
        if has_sector_overlap(elf_1, elf_2) {
            overlaps + 1
        } else {
            overlaps
        }
    });

    println!("{}", contains_count);
    println!("{}", overlap_count);
}

fn has_sector_contains(elf_1: &str, elf_2: &str) -> bool {
    let (elf_1_start, elf_1_end) = match elf_1.split_once("-") {
        Some(sectors) => (
            sectors.0.parse::<u32>().unwrap(),
            sectors.1.parse::<u32>().unwrap(),
        ),
        None => return false,
    };
    let (elf_2_start, elf_2_end) = match elf_2.split_once("-") {
        Some(sectors) => (
            sectors.0.parse::<u32>().unwrap(),
            sectors.1.parse::<u32>().unwrap(),
        ),
        None => return false,
    };

    return (elf_1_start <= elf_2_start && elf_1_end >= elf_2_end)
        || (elf_2_start <= elf_1_start && elf_2_end >= elf_1_end);
}

fn has_sector_overlap(elf_1: &str, elf_2: &str) -> bool {
    let (elf_1_start, elf_1_end) = match elf_1.split_once("-") {
        Some(sectors) => (
            sectors.0.parse::<u32>().unwrap(),
            sectors.1.parse::<u32>().unwrap(),
        ),
        None => return false,
    };
    let (elf_2_start, elf_2_end) = match elf_2.split_once("-") {
        Some(sectors) => (
            sectors.0.parse::<u32>().unwrap(),
            sectors.1.parse::<u32>().unwrap(),
        ),
        None => return false,
    };

    return elf_1_start <= elf_2_end && elf_2_start <= elf_1_end
}
