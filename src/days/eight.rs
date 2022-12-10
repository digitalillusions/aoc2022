use std::fs;

pub fn treetop_tree_house() {
    println!("Running day 8");
    let contents = fs::read_to_string("sample_files/08/example.txt").unwrap();
    // (height, visible, view distance to [top, left, bottom, right])
    let mut grid: Vec<Vec<(u32, bool, [u32; 4])>> = Vec::from(Vec::new());

    for line in contents.lines() {
        let mut row = Vec::new();
        for c in line.chars().map(|c| c.to_digit(10).unwrap()) {
            row.push((c, false, [0, 0, 0, 0]));
        }
        grid.push(row);
    }

    let n_rows = grid.len();
    let n_columns = grid.first().unwrap().len();

    let mut max_left;
    let mut max_bottom = Vec::from_iter(
        grid.first()
            .unwrap()
            .iter()
            .cloned()
            .map(|(height, _, _)| height),
    );

    let mut view_left: (u32, u32);
    let mut view_bottom = Vec::new();
    view_bottom.resize(n_columns, (10, 0));
    println!("{:?}", view_bottom);

    // Check visibility from left and bottom
    for (i, row) in grid.iter_mut().enumerate() {
        max_left = row.first().unwrap().0;
        view_left = (10, 0);
        for (j, (height, visible, view_distance)) in row.iter_mut().enumerate() {
            // Mark border of forest
            if i == 0 || i == n_rows || j == 0 || j == n_columns {
                *visible = true;
            }

            // Check visibility from left
            if *height > max_left {
                *visible = true;
                max_left = *height;
            } else if *height >= view_left.0 {
                view_left.0 = *height;
                view_left.1 += 1;
                view_distance[1] = view_left.1;
            } else {
                view_left.0 = *height;
                view_left.1 = 0;
                view_distance[1] = 0;
            }

            // Update visibility from bottom
            if *height > *max_bottom.get(j).unwrap() {
                *visible = true;
                *max_bottom.get_mut(j).unwrap() = *height;
            } else if *height >= view_bottom.get(j).unwrap().0 {
                view_bottom.get_mut(j).unwrap().0 = *height;
                view_bottom.get_mut(j).unwrap().1 += 1;
                view_distance[2] = view_bottom.get(j).unwrap().1;
            } else {
                view_bottom.get_mut(j).unwrap().0 = *height;
                view_bottom.get_mut(j).unwrap().1 = 0;
                view_distance[2] = 0;
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
    // Check visibility from right an top
    for (i, row) in grid.iter_mut().rev().enumerate() {
        println!("{}", i);
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

    for row in grid {
        println!("{:?}", row);
    }

    println!("\tPart 1: {}", n_visible);
}
