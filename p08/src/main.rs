use std::usize;

#[derive(Debug)]
struct Tree {
    height: Option<u32>,
    top: Option<u32>,
    right: Option<u32>,
    bottom: Option<u32>,
    left: Option<u32>,
}

fn main() {
    let input = include_str!("./input.txt");
    let rows = input
        .split("\n")
        .filter(|r| !r.is_empty())
        .collect::<Vec<&str>>();
    let trees = parse_trees(rows);
}

fn parse_trees(tree_vec: Vec<&str>) -> Vec<Tree> {
    let mut trees = Vec::new();
    for (row_idx, row) in tree_vec.iter().enumerate() {
        for (col_idx, tree) in row.chars().enumerate() {
            //println!("row: {}, col: {}, height: {}", row_idx, col_idx, tree);
            trees.push(Tree {
                height: tree.to_digit(10),
                top: get_tree_top(&tree_vec, row_idx, col_idx),
                right: get_tree_right(&tree_vec, row_idx, col_idx),
                bottom: get_tree_bottom(&tree_vec, row_idx, col_idx),
                left: get_tree_left(&tree_vec, row_idx, col_idx),
            });
        }
    }
    trees
}

fn get_tree_top(tree_vec: &Vec<&str>, row_idx: usize, col_idx: usize) -> Option<u32> {
    tree_vec
        .get(row_idx.checked_sub(1)?)?
        .chars()
        .nth(col_idx)?
        .to_digit(10)
}

fn get_tree_right(tree_vec: &Vec<&str>, row_idx: usize, col_idx: usize) -> Option<u32> {
    tree_vec
        .get(row_idx)?
        .chars()
        .nth(col_idx + 1)?
        .to_digit(10)
}

fn get_tree_bottom(tree_vec: &Vec<&str>, row_idx: usize, col_idx: usize) -> Option<u32> {
    tree_vec
        .get(row_idx + 1)?
        .chars()
        .nth(col_idx)?
        .to_digit(10)
}

fn get_tree_left(tree_vec: &Vec<&str>, row_idx: usize, col_idx: usize) -> Option<u32> {
    tree_vec
        .get(row_idx)?
        .chars()
        .nth(col_idx.checked_sub(1)?)?
        .to_digit(10)
}

fn print_trees(trees: Vec<Tree>) {
    for tree in trees {
        println!(
            "HEIGHT: {:?}, TOP: {:?}, RIGHT: {:?}, BOTTOM: {:?}, LEFT: {:?}",
            tree.height, tree.top, tree.right, tree.bottom, tree.left
        );
    }
}
