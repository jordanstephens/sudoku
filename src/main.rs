mod grid;
mod util;

use std::env;
use std::fs;
use std::path::Path;
use grid::Grid;

type SResult<T> = std::io::Result<T>;

struct Puzzle {
    grid: Grid,
}

impl Puzzle {
    fn from_string(input: &String) -> Self {
        Puzzle {
            grid: Grid::from_string(input),
        }
    }

    fn solve(&self) -> Option<Grid> {
        search(&self.grid)
    }
}

fn search(grid: &Grid) -> Option<Grid> {
    if grid.is_solved() { return Some(grid.clone()); };
    if let Some(next) = select_next(&grid) {
        let possible_vals = grid.at(next);
        for val in possible_vals.chars() {
            let mut clone = grid.clone();
            let succeeded = clone.set(next, &val.to_string(), 0);
            if !succeeded { return None }
            let result = search(&clone);
            if let Some(solution) = result {
                return Some(solution);
            }
        }
    }

    None
}

fn select_next(grid: &Grid) -> Option<usize> {
    grid
        .all_values()
        .enumerate()
        .map(|(i, vals)| (i, vals.len()))
        .filter(|&(_s, len)| len > 1)
        .min_by_key(|&(_s, len)| len)
        .map(|(next, _)| next)
}

fn main() -> SResult<()> {
    let paths: Vec<String> = env::args().skip(1).collect();
    run(&paths)?;
    Ok(())
}

fn run(paths: &Vec<String>) -> SResult<()> {
    let _solutions: Vec<SResult<Option<Grid>>> = paths
        .iter()
        .map(|path| -> SResult<Option<Grid>> {
            let puzzle = load_puzzle(path)?;
            println!("{}", puzzle.grid.to_string());
            match puzzle.solve() {
                Some(solution) => {
                    println!("Solved!\n{}", solution.to_string());
                    return Ok(Some(solution));
                },
                None => {
                    println!("Unable to solve\n{}", puzzle.grid.to_string());
                    return Ok(None);
                }
            }
        }).collect();

    Ok(())
}

fn load_puzzle<P: AsRef<Path>>(path: P) -> SResult<Puzzle> {
    let puzzle_str = fs::read_to_string(path)?;
    Ok(Puzzle::from_string(&puzzle_str))
}
