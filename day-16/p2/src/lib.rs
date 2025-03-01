use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

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
pub struct State {
    x: usize,
    y: usize,
    dir: Direction,
}

// Simple Manhattan distance heuristic
fn heuristic(x: usize, y: usize, goal_x: usize, goal_y: usize) -> usize {
    ((x as isize - goal_x as isize).abs() + (y as isize - goal_y as isize).abs()) as usize
}

// Reconstruct all paths from `state` back to `start` using the multi-parent map.
fn reconstruct_paths(
    state: State,
    start: State,
    parents: &HashMap<State, Vec<State>>,
) -> Vec<Vec<State>> {
    if state == start {
        return vec![vec![start]];
    }
    let mut paths = Vec::new();
    if let Some(pars) = parents.get(&state) {
        for &parent in pars {
            for mut path in reconstruct_paths(parent, start, parents) {
                path.push(state);
                paths.push(path);
            }
        }
    }
    paths
}

// Solve the maze, returning the best cost and a list of all optimal paths.
// Each path is a list of states from start to goal.
pub fn solve_maze_all(maze: &[&str]) -> Option<(usize, Vec<Vec<State>>)> {
    let height = maze.len();
    let width = maze[0].len();

    // Locate start and goal.
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

    let mut heap = BinaryHeap::new();
    let start_state = State {
        x: start_x,
        y: start_y,
        dir: Direction::East,
    };
    let mut dist: HashMap<State, usize> = HashMap::new();
    // Use a multi-parent map to record all optimal ways to reach a state.
    let mut parents: HashMap<State, Vec<State>> = HashMap::new();

    dist.insert(start_state, 0);
    heap.push(Reverse((
        heuristic(start_x, start_y, goal_x, goal_y),
        0,
        start_state,
    )));

    // Keep track of all goal states found (with optimal cost) and the best cost.
    let mut best_cost: Option<usize> = None;
    let mut goal_states = Vec::new();

    while let Some(Reverse((_, cost, state))) = heap.pop() {
        // If we already have a solution and this state's cost exceeds it, we can stop.
        if let Some(best) = best_cost {
            if cost > best {
                break;
            }
        }

        // Check if this state is a goal.
        if state.x == goal_x && state.y == goal_y {
            best_cost = Some(cost);
            goal_states.push(state);
            // Do not return immediately; there might be other goal states with equal cost.
            continue;
        }

        // If we have found a better cost before for this state, skip it.
        if let Some(&known_cost) = dist.get(&state) {
            if cost > known_cost {
                continue;
            }
        }

        // Try turning left and right.
        for &new_dir in &[state.dir.left(), state.dir.right()] {
            let new_state = State {
                x: state.x,
                y: state.y,
                dir: new_dir,
            };
            let new_cost = cost + 1000;
            let entry = dist.entry(new_state).or_insert(usize::MAX);
            if new_cost < *entry {
                *entry = new_cost;
                parents.insert(new_state, vec![state]);
                let new_priority = new_cost + heuristic(state.x, state.y, goal_x, goal_y);
                heap.push(Reverse((new_priority, new_cost, new_state)));
            } else if new_cost == *entry {
                // Add an additional parent if the same cost is achieved.
                parents.entry(new_state).or_default().push(state);
            }
        }

        // Try moving forward.
        let (dx, dy) = state.dir.dx_dy();
        let new_x = state.x as isize + dx;
        let new_y = state.y as isize + dy;
        if new_x >= 0 && new_y >= 0 && new_x < width as isize && new_y < height as isize {
            let new_x = new_x as usize;
            let new_y = new_y as usize;
            if maze[new_y].chars().nth(new_x).unwrap() != '#' {
                let new_state = State {
                    x: new_x,
                    y: new_y,
                    dir: state.dir,
                };
                let new_cost = cost + 1;
                let entry = dist.entry(new_state).or_insert(usize::MAX);
                if new_cost < *entry {
                    *entry = new_cost;
                    parents.insert(new_state, vec![state]);
                    let new_priority = new_cost + heuristic(new_x, new_y, goal_x, goal_y);
                    heap.push(Reverse((new_priority, new_cost, new_state)));
                } else if new_cost == *entry {
                    parents.entry(new_state).or_default().push(state);
                }
            }
        }
    }

    if let Some(best) = best_cost {
        // Reconstruct all paths from every goal state.
        let mut all_paths = Vec::new();
        for goal_state in goal_states {
            let mut paths = reconstruct_paths(goal_state, start_state, &parents);
            // Each path is currently from start to goal.
            all_paths.append(&mut paths);
        }
        Some((best, all_paths))
    } else {
        None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn from_state(state: State) -> Self {
        Pos {
            x: state.x,
            y: state.y,
        }
    }
}

#[allow(dead_code)]
fn state_to_pos(state_history: Vec<State>) -> Vec<Pos> {
    let mut v: Vec<Pos> = Vec::new();
    let mut last: Option<Pos> = None;
    for state in state_history {
        let new = Pos::from_state(state);
        if Some(new) != last {
            last = Some(new);
            v.push(new);
        }
    }
    v
}

fn unique_positions(paths: Vec<Vec<State>>) -> Vec<Pos> {
    let mut positions: HashSet<Pos> = HashSet::new();
    for path in paths {
        for state in path {
            positions.insert(Pos::from_state(state));
        }
    }
    Vec::from_iter(positions)
}

#[allow(dead_code)]
fn print_path(maze: &[&str], path: Vec<Pos>) {
    let mut vec_maze: Vec<Vec<char>> = maze.iter().map(|line| line.chars().collect()).collect();
    for pos in path {
        vec_maze[pos.y][pos.x] = 'O';
    }
    for line in &vec_maze {
        println!("{}", line.iter().collect::<String>());
    }
}

pub fn find_num_unique_tiles(maze: &[&str]) -> usize {
    let states = solve_maze_all(&maze);
    if states == None {
        panic!("No solutions found for give maze.");
    }
    let unique_pos_vec = unique_positions(states.unwrap().1);
    unique_pos_vec.len()
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

        let result = solve_maze_all(&maze).expect("No solution found.").0;
        let solution: usize = 7036;
        assert_eq!(result, solution);
    }

    #[test]
    fn test_path1() {
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

        let result = solve_maze_all(&maze).expect("No solution found.").1;
        let result: Vec<Pos> = unique_positions(result);
        print_path(&maze, result.clone());
        let solution: usize = 45;
        assert_eq!(result.len(), solution);
    }

    #[test]
    fn test_path2() {
        let input = r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

        let maze = parse_input(input);

        let result = solve_maze_all(&maze).expect("No solution found.").1;
        let result: Vec<Pos> = unique_positions(result);
        print_path(&maze, result.clone());
        let solution: usize = 64;
        assert_eq!(result.len(), solution);
    }

    #[test]
    fn test_find_unique_tiles_num() {
        let input = r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

        let maze = parse_input(input);

        let result = find_num_unique_tiles(&maze);
        let solution: usize = 64;
        assert_eq!(result, solution);
    }
}
