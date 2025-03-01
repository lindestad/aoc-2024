use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub fn parse_input(input: &str) -> Vec<&str> {
    input.lines().filter(|&line| line != "\n").collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    // Rotate 90 degrees to the left
    fn left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    // Rotate 90 degrees to the right
    fn right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    // Get the (dx, dy) vector for moving forward in this direction
    fn dx_dy(self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct State {
    x: usize,
    y: usize,
    dir: Direction,
}

// Simple Manhattan distance heuristic
fn heuristic(x: usize, y: usize, goal_x: usize, goal_y: usize) -> usize {
    ((x as isize - goal_x as isize).abs() + (y as isize - goal_y as isize).abs()) as usize
}

pub fn solve_maze(maze: &[&str]) -> Option<usize> {
    let height = maze.len();
    let width = maze[0].len();

    // Locate the start (S) and end (E) positions.
    let mut start = None;
    let mut goal = None;
    for (y, line) in maze.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start = Some((x, y));
            } else if ch == 'E' {
                goal = Some((x, y));
            }
        }
    }

    let (start_x, start_y) = start?;
    let (goal_x, goal_y) = goal?;

    // A* search setup
    let mut heap = BinaryHeap::new();
    let start_state = State {
        x: start_x,
        y: start_y,
        dir: Direction::East, // Start facing East
    };
    // Track the lowest cost to each state.
    let mut dist: HashMap<State, usize> = HashMap::new();
    dist.insert(start_state, 0);

    // The heap holds (priority, cost, state). Reverse is used because BinaryHeap is a max-heap.
    heap.push(Reverse((
        heuristic(start_x, start_y, goal_x, goal_y),
        0,
        start_state,
    )));

    while let Some(Reverse((_, cost, state))) = heap.pop() {
        // Check if we have reached the goal (direction doesn't matter)
        if state.x == goal_x && state.y == goal_y {
            return Some(cost);
        }

        // Skip if we already found a better way to this state.
        if let Some(&known_cost) = dist.get(&state) {
            if cost > known_cost {
                continue;
            }
        }

        // Consider turning left and right.
        for &new_dir in &[state.dir.left(), state.dir.right()] {
            let new_state = State {
                x: state.x,
                y: state.y,
                dir: new_dir,
            };
            let new_cost = cost + 1000;
            if new_cost < *dist.get(&new_state).unwrap_or(&usize::MAX) {
                dist.insert(new_state, new_cost);
                let new_priority = new_cost + heuristic(state.x, state.y, goal_x, goal_y);
                heap.push(Reverse((new_priority, new_cost, new_state)));
            }
        }

        // Consider moving forward.
        let (dx, dy) = state.dir.dx_dy();
        let new_x = state.x as isize + dx;
        let new_y = state.y as isize + dy;
        if new_x >= 0 && new_y >= 0 && new_x < width as isize && new_y < height as isize {
            let new_x = new_x as usize;
            let new_y = new_y as usize;
            // Only move if the next cell is not a wall.
            if maze[new_y].chars().nth(new_x).unwrap() != '#' {
                let new_state = State {
                    x: new_x,
                    y: new_y,
                    dir: state.dir,
                };
                let new_cost = cost + 1;
                if new_cost < *dist.get(&new_state).unwrap_or(&usize::MAX) {
                    dist.insert(new_state, new_cost);
                    let new_priority = new_cost + heuristic(new_x, new_y, goal_x, goal_y);
                    heap.push(Reverse((new_priority, new_cost, new_state)));
                }
            }
        }
    }

    // No path found.
    None
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_solve_maze() {
        let input = r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

        let maze = parse_input(input);

        let result = solve_maze(&maze).expect("No solution found.");
        let solution: usize = 7036;
        assert_eq!(result, solution);
    }
}
