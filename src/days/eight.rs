use std::{fs, ops::Sub};

pub fn treetop_tree_house() {
    println!("Running day 8");
    let contents = fs::read_to_string("sample_files/08/sample.txt").unwrap();
    let mut grid: Vec<Vec<(u32, bool, [u32; 4])>> = Vec::from(Vec::new());

    for line in contents.lines() {
        let mut row = Vec::new();
        for c in line.chars().map(|c| c.to_digit(10).unwrap()) {
            row.push((c, false, [0, 0, 0, 0]));
        }
        grid.push(row);
    }

    let n_rows = grid.len() - 1;
    let n_columns = grid.first().unwrap().len() - 1;

    let mut max_left;
    let mut max_bottom = Vec::from_iter(
        grid.first()
            .unwrap()
            .iter()
            .cloned()
            .map(|(height, _, _)| height),
    );
    // Check visibility
    for (i, row) in grid.iter_mut().enumerate() {
        max_left = row.first().unwrap().0;
        for (j, (height, visible, view_distance)) in row.iter_mut().enumerate() {
            // Mark border of forest
            if i == 0 || i == n_rows || j == 0 || j == n_columns {
                *visible = true;
            }

            // Check visibility from left
            if *height > max_left {
                *visible = true;
                max_left = *height;
            }

            // Update visibility from bottom
            if *height > *max_bottom.get(j).unwrap() {
                *visible = true;
                *max_bottom.get_mut(j).unwrap() = *height;
            }
        }
    }

    let mut max_right;
    let mut max_top = Vec::from_iter(
        grid.last()
            .unwrap()
            .iter()
            .rev()
            .cloned()
            .map(|(height, _, _)| height),
    );
    // Check visibility
    for (i, row) in grid.iter_mut().rev().enumerate() {
        max_right = row.last().unwrap().0;
        for (j, (height, visible, view_distance)) in row.iter_mut().rev().enumerate() {
            // Check visibility from right
            if *height > max_right {
                *visible = true;
                max_right = *height;
            }

            // Update visibility from bottom
            if *height > *max_top.get(j).unwrap() {
                *visible = true;
                *max_top.get_mut(j).unwrap() = *height;
            }
        }
    }

    let n_visible = grid
        .iter()
        .flatten()
        .filter(|(_, visible, _)| *visible)
        .count();

    println!("\tPart 1: {}", n_visible);

    let to_check = grid
        .iter()
        .enumerate()
        .flat_map(|(i, inner_vec)| {
            inner_vec
                .iter()
                .enumerate()
                .map(move |(j, item)| (i, j, item))
        })
        .filter_map(|(i, j, item)| if item.1 { Some((i, j)) } else { None })
        .collect::<Vec<_>>();

    for (x, y) in to_check.iter() {
        let (height, _, _) = grid.get(*x).unwrap().get(*y).unwrap().clone();
        let mut visibility = 0;
        for i in *x + 1..n_rows + 1 {
            let (h, _, _) = grid.get(i).unwrap().get(*y).unwrap();
            if *h < height {
                visibility += 1;
            } else {
                visibility += 1;
                break;
            }
        }
        grid.get_mut(*x).unwrap().get_mut(*y).unwrap().2[2] = visibility;

        visibility = 0;
        for i in (0..*x).rev() {
            let (h, _, _) = grid.get(i).unwrap().get(*y).unwrap();
            if *h < height {
                visibility += 1;
            } else {
                visibility += 1;
                break;
            }
        }
        grid.get_mut(*x).unwrap().get_mut(*y).unwrap().2[0] = visibility;

        visibility = 0;
        for j in *y + 1..n_columns + 1 {
            let (h, _, _) = grid.get(*x).unwrap().get(j).unwrap();
            if *h < height {
                visibility += 1;
            } else {
                visibility += 1;
                break;
            }
        }
        grid.get_mut(*x).unwrap().get_mut(*y).unwrap().2[3] = visibility;

        visibility = 0;
        for j in (0..*y).rev() {
            let (h, _, _) = grid.get(*x).unwrap().get(j).unwrap();
            if *h < height {
                visibility += 1;
            } else {
                visibility += 1;
                break;
            }
        }
        grid.get_mut(*x).unwrap().get_mut(*y).unwrap().2[1] = visibility;
    }

    let max_score = grid
        .iter()
        .flatten()
        .map(|(_, _, v)| v[0] * v[1] * v[2] * v[3])
        .max()
        .unwrap();

    println!("\tPart 2: {}", max_score);
}
