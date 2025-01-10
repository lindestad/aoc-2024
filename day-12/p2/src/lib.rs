use std::collections::HashSet;

pub fn str_to_vec(input: &str) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            if c != '\r' && c != '\n' {
                row.push(c);
            }
        }
        if !row.is_empty() {
            output.push(row);
        }
    }
    output
}

fn check_inbounds(x: i32, y: i32, width: i32, height: i32) -> bool {
    x >= 0 && x < width && y >= 0 && y < height
}

pub fn find_single_area(
    x: usize,
    y: usize,
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize)>,
) -> usize {
    let letter = grid[y][x];
    let mut area = 0;
    let mut stack = vec![(x, y)];
    while !stack.is_empty() {
        let (x, y) = stack.pop().unwrap();
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        area += 1;
        for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            if check_inbounds(new_x, new_y, grid[0].len() as i32, grid.len() as i32)
                && grid[new_y as usize][new_x as usize] == letter
            {
                stack.push((new_x as usize, new_y as usize));
            }
        }
    }
    area
}

pub fn calculate_single_price(
    x: usize,
    y: usize,
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize)>,
) -> i32 {
    let letter = grid[y][x];
    let mut area = 0;
    let mut fence = 0;
    let mut stack = vec![(x, y)];
    while !stack.is_empty() {
        let (x, y) = stack.pop().unwrap();
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        area += 1;
        for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            if check_inbounds(new_x, new_y, grid[0].len() as i32, grid.len() as i32)
                && grid[new_y as usize][new_x as usize] == letter
            {
                stack.push((new_x as usize, new_y as usize));
            } else {
                fence += 1;
            }
        }
    }
    area * fence
}

pub fn calculate_price(input: &str) -> i32 {
    let grid = str_to_vec(input);
    let mut visited = HashSet::new();
    let mut price = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if !visited.contains(&(x, y)) {
                price += calculate_single_price(x, y, &grid, &mut visited);
            }
        }
    }
    price
}

// pub fn find_edge(x: usize, y: usize, grid: &Vec<Vec<char>>) -> Vec<usize> {
//     let letter = grid[y][x];
//     let mut edges = vec![];
//     for (i, (dx, dy)) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)].enumerate() {
//         let new_x = x as i32 + dx;
//         let new_y = y as i32 + dy;
//         if check_inbounts(new_x, new_y, grid[0].len() as i32, grid.len() as i32)
//             && grid[new_y as usize][new_x as usize] != letter
//         {
//             edges.push(grid[new_y as usize][new_x as usize] as usize);
//         }
//     }
//     edges
// }

// pub fn find_edges(
//     x: usize,
//     y: usize,
//     grid: &Vec<Vec<char>>,
//     &mut visited: HashSet<(usize, usize)>,
// ) -> HashMap<(usize, usize), Vec<usize>> {
//     // Tread the grid (2D surface) as a surface with topoloigically
//     // embedded structures. The edges are the points where the surface
//     // is not flat, i.e. we embed them in the z axis. There can be multiple
//     // edges at the same point, so we store a vector of them for each point
//     // in the grid. We map the edges as a closed loop 1-4, where each neighboring
//     // number is connected by an edge, we can then search for the neighboring edge
//     // for any given edge as i _must_! exist. 1 connects to 2 or 4, etc.
//     // The edges are stored as a tuple HashMap<(usize, usize), Vec<usize>>
//     // where the key is the point in the grid and the value is the vector of edges.
//     // a flat side is represented as a 0, no sides (internal) is not stored.
//     let letter = grid[y][x];
//     let mut edges: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
//     let mut stack = vec![(x, y)];
//     while !stack.is_empty() {
//         let (x, y) = stack.pop().unwrap();
//         if visited.contains(&(x, y)) {
//             continue;
//         }
//         visited.insert((x, y));
//         area += 1;
//         for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
//             let new_x = x as i32 + dx;
//             let new_y = y as i32 + dy;
//             if check_inbounts(new_x, new_y, grid[0].len() as i32, grid.len() as i32)
//                 && grid[new_y as usize][new_x as usize] == letter
//             {
//                 stack.push((new_x as usize, new_y as usize));
//             } else {
//                 fence += 1;
//             }
//         }
//     }

//     area * fence
// }

// pub fn count_corners(grid: &Vec<Vec<char>>) -> usize {
//     let mut corners: usize = 0;
//     let width = grid[0].len();
//     let height = grid.len();
//     // handle small grids
//     if width == 0 {
//         panic!("Empty grid provided");
//     }

//     // should not need this
//     // if width + height == 3 || (width == 1 && height == 1) {
//     //     if grid[0][0] == grid[width - 1][height - 1] {
//     //         corners = 4;
//     //     } else {
//     //         corners = 8;
//     //     }
//     //     return corners;
//     // }

//     // Find perimeter corners to avoid checking validity of usize indexes
//     for x in 0..width - 1 {
//         // top row
//         let letter = grid[0][x];
//         if letter != grid[0][x + 1] {
//             corners += 2;
//         }
//         if height > 1 {
//             // inside corners
//             if x < width - 1 {
//                 // avoid overflow
//                 if letter == grid[1][x] && letter == grid[0][x + 1] && letter != grid[1][x + 1] {
//                     corners += 1;
//                     dbg!(corners, "top row 1");
//                 } else if letter != grid[1][x] && letter != grid[0][x + 1] {
//                     corners += 1;
//                     dbg!(corners, "top row 2");
//                 }
//             }
//         }
//     }
//     for x in 0..width - 1 {
//         // bottom row
//         let letter = grid[height - 1][x];
//         if letter != grid[height - 1][x + 1] {
//             corners += 2;
//         }
//         if height > 1 {
//             // inside corners
//             if x < width - 1 {
//                 // avoid overflow
//                 if letter == grid[height - 2][x]
//                     && letter == grid[height - 1][x + 1]
//                     && letter != grid[height - 2][x + 1]
//                 {
//                     corners += 1;
//                     dbg!(corners, "bottom row 1");
//                 } else if letter != grid[height - 2][x] && letter != grid[height - 1][x + 1] {
//                     corners += 1;
//                     dbg!(corners, "bottom row 2");
//                 }
//             }
//         }
//     }
//     for y in 0..height - 1 {
//         // left column
//         let letter = grid[y][0];
//         if letter != grid[y + 1][0] {
//             corners += 2;
//         }
//         if width > 1 {
//             // inside corners
//             if y < height - 1 {
//                 // avoid overflow
//                 if letter == grid[y][1] && letter == grid[y + 1][0] && letter != grid[y + 1][1] {
//                     corners += 1;
//                     dbg!(corners, "left column 1");
//                 } else if letter != grid[y][1] && letter != grid[y + 1][0] {
//                     corners += 1;
//                     dbg!(corners, "left column 2");
//                 }
//             }
//         }
//     }
//     for y in 0..height - 1 {
//         // right column
//         let letter = grid[y][width - 1];
//         if letter != grid[y + 1][width - 1] {
//             corners += 2;
//         }
//         if width > 1 {
//             // inside corners
//             if y < height - 1 {
//                 // avoid overflow
//                 if letter == grid[y][width - 2]
//                     && letter == grid[y + 1][width - 1]
//                     && letter != grid[y + 1][width - 2]
//                 {
//                     corners += 1;
//                     dbg!(corners, "right column 1");
//                 } else if letter != grid[y][width - 2] && letter != grid[y + 1][width - 1] {
//                     corners += 1;
//                     dbg!(corners, "right column 2");
//                 }
//             }
//         }
//     }
//     corners += 4; // corners of the grid

//     for y in 1..height - 1 {
//         for x in 1..width - 1 {
//             let letter = grid[y][x];
//             let directions = [
//                 [(-1, 0), (0, -1)], // Left & Up
//                 [(1, 0), (0, -1)],  // Right & Up
//                 [(-1, 0), (0, 1)],  // Left & Down
//                 [(1, 0), (0, 1)],   // Right & Down
//             ];
//             for direction in &directions {
//                 let dx1 = direction[0].0;
//                 let dy1 = direction[0].1;
//                 let dx2 = direction[1].0;
//                 let dy2 = direction[1].1;
//                 if letter == grid[(y as i32 + dy1) as usize][(x as i32 + dx1) as usize]
//                     && letter == grid[(y as i32 + dy2) as usize][(x as i32 + dx2) as usize]
//                     && letter
//                         != grid[(y as i32 + dy1 + dy2) as usize][(x as i32 + dx1 + dx2) as usize]
//                 {
//                     // corner like this: _|X
//                     // (exterior)        X X
//                     corners += 1;
//                 } else if letter != grid[(y as i32 + dy1) as usize][(x as i32 + dx1) as usize]
//                     && letter != grid[(y as i32 + dy2) as usize][(x as i32 + dx2) as usize]
//                 {
//                     // corner like this   _
//                     // (interior)        |X
//                     corners += 1;
//                 }
//             }
//         }
//     }
//     corners
// }

pub fn count_corners(grid: &Vec<Vec<char>>) -> usize {
    let mut corners: usize = 0;
    let width = grid[0].len();
    let height = grid.len();
    // handle small grids
    if width == 0 {
        panic!("Empty grid provided");
    }

    for y in 0..height {
        for x in 0..width {
            let letter = grid[y][x];
            let directions = [
                [(-1, 0), (0, -1)], // Left & Up
                [(1, 0), (0, -1)],  // Right & Up
                [(-1, 0), (0, 1)],  // Left & Down
                [(1, 0), (0, 1)],   // Right & Down
            ];
            for direction in &directions {
                let dx1 = direction[0].0;
                let dy1 = direction[0].1;
                let dx2 = direction[1].0;
                let dy2 = direction[1].1;
                if check_inbounds(
                    (x as i32 + dx1),
                    (y as i32 + dy1),
                    width as i32,
                    height as i32,
                ) && check_inbounds(
                    (x as i32 + dx2),
                    (y as i32 + dy2),
                    width as i32,
                    height as i32,
                ) && check_inbounds(
                    (x as i32 + dx1 + dx2),
                    (y as i32 + dy1 + dy2),
                    width as i32,
                    height as i32,
                ) && letter == grid[(y as i32 + dy1) as usize][(x as i32 + dx1) as usize]
                    && letter == grid[(y as i32 + dy2) as usize][(x as i32 + dx2) as usize]
                    && letter
                        != grid[(y as i32 + dy1 + dy2) as usize][(x as i32 + dx1 + dx2) as usize]
                {
                    // corner like this: _|X
                    // (exterior)        X X
                    corners += 1;
                } else if check_inbounds(
                    (x as i32 + dx1),
                    (y as i32 + dy1),
                    width as i32,
                    height as i32,
                ) && check_inbounds(
                    (x as i32 + dx2),
                    (y as i32 + dy2),
                    width as i32,
                    height as i32,
                ) && letter != grid[(y as i32 + dy1) as usize][(x as i32 + dx1) as usize]
                    && letter != grid[(y as i32 + dy2) as usize][(x as i32 + dx2) as usize]
                {
                    // corner like this   _
                    // (interior)        |X
                    corners += 1;
                }
            }
        }
    }
    corners
}
