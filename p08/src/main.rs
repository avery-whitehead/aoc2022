struct Tree {
    height: u32,
    top: Vec<u32>,
    right: Vec<u32>,
    bottom: Vec<u32>,
    left: Vec<u32>,
}

fn main() {
    let input = include_str!("./input.txt");
    let rows = input
        .split("\n")
        .filter(|r| !r.is_empty())
        .collect::<Vec<&str>>();
    let trees = parse_trees(rows);
    // Part 1
    let visible_trees = trees.iter().fold(0, |acc, tree| {
        if is_tree_edge(tree) || is_tree_visible(tree) {
            return acc + 1;
        }
        acc
    });
    println!("{}", visible_trees);
    // Part 2
    let max_tree_score = trees.iter().map(calc_tree_score).max().unwrap_or_default();
    println!("{}", max_tree_score);
}

fn parse_trees(tree_vec: Vec<&str>) -> Vec<Tree> {
    let mut trees = Vec::new();
    for (row_idx, row) in tree_vec.iter().enumerate() {
        for (col_idx, tree) in row.chars().enumerate() {
            trees.push(Tree {
                height: tree.to_digit(10).unwrap_or_default(),
                top: get_trees_top(&tree_vec, row_idx, col_idx).unwrap_or_default(),
                right: get_trees_right(&tree_vec, row_idx, col_idx).unwrap_or_default(),
                bottom: get_trees_bottom(&tree_vec, row_idx, col_idx).unwrap_or_default(),
                left: get_trees_left(&tree_vec, row_idx, col_idx).unwrap_or_default(),
            });
        }
    }
    trees
}

fn get_trees_top(tree_vec: &Vec<&str>, row_idx: usize, col_idx: usize) -> Option<Vec<u32>> {
    tree_vec.get(0..row_idx).map(|rows| {
        rows.iter()
            .rev()
            .map(|row| row.chars().nth(col_idx)?.to_digit(10))
            .flatten()
            .collect()
    })
}

fn get_trees_right(tree_vec: &Vec<&str>, row_idx: usize, col_idx: usize) -> Option<Vec<u32>> {
    tree_vec.get(row_idx).map(|row| {
        row.chars()
            .map(|c| c.to_digit(10))
            .skip(col_idx + 1)
            .flatten()
            .collect()
    })
}

fn get_trees_bottom(tree_vec: &Vec<&str>, row_idx: usize, col_idx: usize) -> Option<Vec<u32>> {
    tree_vec.get(row_idx + 1..tree_vec.len()).map(|rows| {
        rows.iter()
            .map(|row| row.chars().nth(col_idx)?.to_digit(10))
            .flatten()
            .collect()
    })
}

fn get_trees_left(tree_vec: &Vec<&str>, row_idx: usize, col_idx: usize) -> Option<Vec<u32>> {
    tree_vec.get(row_idx).map(|row| {
        row.chars()
            .rev()
            .map(|c| c.to_digit(10))
            .skip(row.len() - col_idx)
            .flatten()
            .collect()
    })
}

fn is_tree_edge(tree: &Tree) -> bool {
    tree.top.is_empty() || tree.right.is_empty() || tree.bottom.is_empty() || tree.left.is_empty()
}

fn is_tree_visible(tree: &Tree) -> bool {
    tree.top.iter().all(|h| &tree.height > h)
        || tree.right.iter().all(|h| &tree.height > h)
        || tree.bottom.iter().all(|h| &tree.height > h)
        || tree.left.iter().all(|h| &tree.height > h)
}

fn calc_tree_score(tree: &Tree) -> usize {
    let top_score = get_trees_until_height(&tree.height, &tree.top);
    let right_score = get_trees_until_height(&tree.height, &tree.right);
    let bottom_score = get_trees_until_height(&tree.height, &tree.bottom);
    let left_score = get_trees_until_height(&tree.height, &tree.left);
    top_score * right_score * bottom_score * left_score
}

fn get_trees_until_height(height: &u32, trees: &Vec<u32>) -> usize {
    let mut count = 0;
    for tree_height in trees {
        count += 1;
        if tree_height >= height {
            break;
        }
    }
    count
}
