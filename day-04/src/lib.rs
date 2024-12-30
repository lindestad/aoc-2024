pub fn count_xmas(grid: Vec<Vec<char>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let word = ['X', 'M', 'A', 'S'];
    let mut count = 0;

    // Check for XMAS in all orientations
    for i in 0..rows {
        for j in 0..cols {
            // Horizontal (left-to-right)
            if j + 3 < cols && grid[i][j..j + 4] == word {
                count += 1;
            }
            // Horizontal (right-to-left)
            if j >= 3 && grid[i][j - 3..=j].iter().rev().eq(word.iter()) {
                count += 1;
            }
            // Vertical (top-to-bottom)
            if i + 3 < rows && (0..4).all(|k| grid[i + k][j] == word[k]) {
                count += 1;
            }
            // Vertical (bottom-to-top)
            if i >= 3 && (0..4).all(|k| grid[i - k][j] == word[k]) {
                count += 1;
            }
            // Diagonal (top-left to bottom-right)
            if i + 3 < rows && j + 3 < cols && (0..4).all(|k| grid[i + k][j + k] == word[k]) {
                count += 1;
            }
            // Diagonal (bottom-right to top-left)
            if i >= 3 && j >= 3 && (0..4).all(|k| grid[i - k][j - k] == word[k]) {
                count += 1;
            }
            // Diagonal (top-right to bottom-left)
            if i + 3 < rows && j >= 3 && (0..4).all(|k| grid[i + k][j - k] == word[k]) {
                count += 1;
            }
            // Diagonal (bottom-left to top-right)
            if i >= 3 && j + 3 < cols && (0..4).all(|k| grid[i - k][j + k] == word[k]) {
                count += 1;
            }
        }
    }

    return count;
}

pub fn count_mas_crosses(grid: Vec<Vec<char>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let word = ['M', 'A', 'S'];
    let word_reversed = word.iter().copied().rev().collect::<Vec<char>>();
    let mut count = 0;

    // Check for MAS cross in all diagonal orientations
    for i in 0..rows {
        for j in 0..cols {
            let mut found = 0;
            // Diagonal (top-left to bottom-right)
            if i + 2 < rows
                && j + 2 < cols
                && ((0..3).all(|k| grid[i + k][j + k] == word[k])
                    || (0..3).all(|k| grid[i + k][j + k] == word_reversed[k]))
            {
                found += 1;
            }
            // Diagonal (bottom-left to top-right)
            if i + 2 < rows
                && j + 2 < cols
                && ((0..3).all(|k| grid[i + 2 - k][j + k] == word[k])
                    || (0..3).all(|k| grid[i + 2 - k][j + k] == word_reversed[k]))
            {
                found += 1;
            }

            if found == 2 {
                count += 1;
            }
        }
    }

    return count;
}
