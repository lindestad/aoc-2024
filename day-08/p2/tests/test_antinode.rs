use p2::*;
use std::collections::{HashMap, HashSet};
use std::fs;

fn get_test_grid1() -> Vec<Vec<char>> {
    let input1: String =
        fs::read_to_string("tests/test_grids/grid1.txt").expect("Failed to read grid1.txt");
    return input_to_vec(&input1);
}

fn get_test_grid2() -> Vec<Vec<char>> {
    let input2: String =
        fs::read_to_string("tests/test_grids/grid2.txt").expect("Failed to read grid2.txt");
    return input_to_vec(&input2);
}

fn get_test_grid1_solution() -> Vec<Vec<char>> {
    let input1_solution: String = fs::read_to_string("tests/test_grids/grid1_solution.txt")
        .expect("Failed to read grid1_solution.txt");
    return input_to_vec(&input1_solution);
}

fn get_test_grid2_solution() -> Vec<Vec<char>> {
    let input2_solution: String = fs::read_to_string("tests/test_grids/grid2_solution.txt")
        .expect("Failed to read grid2_solution.txt");
    return input_to_vec(&input2_solution);
}

fn get_test_grid_simple1() -> Vec<Vec<char>> {
    let input_simple1: String = fs::read_to_string("tests/test_grids/grid_simple1.txt")
        .expect("Failed to read grid_simple1.txt");
    return input_to_vec(&input_simple1);
}

fn get_test_grid_simple1_solution() -> Vec<Vec<char>> {
    let input_simple1_solution: String =
        fs::read_to_string("tests/test_grids/grid_simple1_solution.txt")
            .expect("Failed to read grid_simple1_solution.txt");
    return input_to_vec(&input_simple1_solution);
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn print_grid_sidebyside(grid1: &Vec<Vec<char>>, grid2: &Vec<Vec<char>>) {
    for (row1, row2) in grid1.iter().zip(grid2.iter()) {
        for (c1, c2) in row1.iter().zip(row2.iter()) {
            print!("{}", c1);
        }
        print!(" | ");
        for (c1, c2) in row1.iter().zip(row2.iter()) {
            print!("{}", c2);
        }
        println!();
    }
    println!();
}

fn overlay_antinodes_on_grid(
    grid: &Vec<Vec<char>>,
    antinodes: HashSet<(usize, usize)>,
) -> Vec<Vec<char>> {
    let mut grid_with_antinodes = grid.clone();
    for (x, y) in antinodes {
        if grid_with_antinodes[y][x] == '.' {
            grid_with_antinodes[y][x] = '#';
        }
    }
    return grid_with_antinodes;
}

#[test]
fn test_find_pair() {
    let grid = get_test_grid1();
    let pairs = find_pairs(&grid);
    let expected_pairs: HashMap<char, Vec<(usize, usize)>> =
        HashMap::from([('T', vec![(0, 0), (3, 1), (1, 2)])]);
    print_grid(&grid);
    assert_eq!(pairs, expected_pairs);
}

#[test]
fn test_simple_grid() {
    let grid = get_test_grid_simple1();
    let solution = get_test_grid_simple1_solution();
    println!(
        "Input:{:<width$}Solution:",
        "",
        width = grid.len() - "Input:".len() + 3
    );
    print_grid_sidebyside(&grid, &solution);

    let pairs = find_pairs(&grid);
    let expected_pairs: HashMap<char, Vec<(usize, usize)>> =
        HashMap::from([('T', vec![(0, 0), (1, 2)])]);
    assert_eq!(pairs, expected_pairs);
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for pair in pairs.iter() {
        let a = find_antinodes_for_pair(
            pair.1[0].0 as i32,
            pair.1[0].1 as i32,
            pair.1[1].0 as i32,
            pair.1[1].1 as i32,
            grid.len(),
            grid[0].len(),
        );
        antinodes.extend(a.into_iter().map(|(x, y)| (x as usize, y as usize)));
    }
    let grid_with_antinodes = overlay_antinodes_on_grid(&grid, antinodes.clone());
    println!(
        "Input:{:<width$}Result:",
        "",
        width = grid.len() - "Input:".len() + 3
    );
    print_grid_sidebyside(&grid, &grid_with_antinodes);
    assert_eq!(grid_with_antinodes, solution);

    println!("Expected number of antinodes: {}", 5);
    println!("Computed number of antinodes: {}", antinodes.len());
    assert_eq!(5, antinodes.len());
}

#[test]
fn test_simple_grid2() {
    let grid = get_test_grid_simple1();
    let solution = get_test_grid_simple1_solution();
    println!(
        "Input:{:<width$}Solution:",
        "",
        width = grid.len() - "Input:".len() + 3
    );
    print_grid_sidebyside(&grid, &solution);

    let antinodes = find_unique_antinodes(&grid);
    let grid_with_antinodes = overlay_antinodes_on_grid(&grid, antinodes.clone());
    println!(
        "Input:{:<width$}Result:",
        "",
        width = grid.len() - "Input:".len() + 3
    );
    print_grid_sidebyside(&grid, &grid_with_antinodes);
    assert_eq!(grid_with_antinodes, solution);

    println!("Expected number of antinodes: {}", 5);
    println!("Computed number of antinodes: {}", antinodes.len());
    assert_eq!(5, antinodes.len());
}

#[test]
fn test_grid1() {
    let grid = get_test_grid1();
    let solution = get_test_grid1_solution();
    println!(
        "Input:{:<width$}Solution:",
        "",
        width = grid.len() - "Input:".len() + 3
    );
    print_grid_sidebyside(&grid, &solution);

    let antinodes = find_unique_antinodes(&grid);
    let grid_with_antinodes = overlay_antinodes_on_grid(&grid, antinodes.clone());
    println!(
        "Input:{:<width$}Result:",
        "",
        width = grid.len() - "Input:".len() + 3
    );
    print_grid_sidebyside(&grid, &grid_with_antinodes);
    assert_eq!(grid_with_antinodes, solution);

    println!("Expected number of antinodes: {}", 9);
    println!("Computed number of antinodes: {}", antinodes.len());
    assert_eq!(9, antinodes.len());
}

#[test]
fn test_grid2() {
    let grid = get_test_grid2();
    let solution = get_test_grid2_solution();
    println!(
        "Input:{:<width$}Solution:",
        "",
        width = grid.len() - "Input:".len() + 3
    );
    print_grid_sidebyside(&grid, &solution);

    let antinodes = find_unique_antinodes(&grid);
    let grid_with_antinodes = overlay_antinodes_on_grid(&grid, antinodes.clone());
    println!(
        "Input:{:<width$}Result:",
        "",
        width = grid.len() - "Input:".len() + 3
    );
    print_grid_sidebyside(&grid, &grid_with_antinodes);
    assert_eq!(grid_with_antinodes, solution);

    println!("Expected number of antinodes: {}", 34);
    println!("Computed number of antinodes: {}", antinodes.len());
    assert_eq!(34, antinodes.len());
}
