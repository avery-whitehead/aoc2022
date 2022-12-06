use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let part_1_marker = find_marker(input, 4).expect("Failed to find marker");
    let part_2_marker = find_marker(input, 14).expect("Failed to find marker");
    println!("{}", part_1_marker);
    println!("{}", part_2_marker);
}

fn find_marker(stream: &str, marker_size: usize) -> Result<usize, &str> {
    let windows = stream.chars().collect::<Vec<_>>();
    for (idx, w) in windows.windows(marker_size).enumerate() {
        let set = w.into_iter().collect::<HashSet<_>>();
        if set.len() == w.len() {
            return Ok(idx + marker_size);
        }
    };
    return Err("Unable to find marker");
}
