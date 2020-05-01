use std::collections::HashSet;
use std::env;
use std::fs;
use std::iter::repeat;
use std::path::Path;

type SResult<T> = std::io::Result<T>;

const RANGE: [u8; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
const ROWS: [u8; 9] = RANGE;
const COLS: [u8; 9] = RANGE;
const DIGITS: &'static str = r#"123456789"#;
type PossibleValues = String; // ex: "123456789"

fn ix(x: u8, y: u8) -> usize {
    assert!(x < 9);
    assert!(y < 9);
    (y * 9 + x) as usize
}

fn dx(i: usize) -> (u8, u8) {
    assert!(i < 81);
    let i = i as u8;
    let x = i % 9;
    let y = i / 9;
    (x, y)
}

fn col_cells(i: usize) -> Vec<usize> {
    let (x, y) = dx(i);
    COLS.iter().map(|&c| ix(x, c)).collect()
}

fn row_cells(i: usize) -> Vec<usize> {
    let (x, y) = dx(i);
    ROWS.iter().map(|&r| ix(r, y)).collect()
}

fn box_cells(i: usize) -> Vec<usize> {
    let (x, y) = dx(i);
    let box_x = x / 9 * 3;
    let box_y = y / 9 * 3;
    let xrange = box_x..box_x + 3;
    let yrange = box_y..box_y + 3;
    xrange.zip(yrange).map(|(x, y)| ix(x, y)).collect()
}

fn units(i: usize) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    (row_cells(i), col_cells(i), box_cells(i))
}

fn peers(i: usize) -> Vec<usize> {
    let (row, col, boxx) = units(i);
    let mut peers = vec![row, col, boxx]
        .concat()
        .iter()
        .fold(HashSet::new(), |mut acc, &i| {
            acc.insert(i);
            acc
        })
        .iter()
        .map(|x| *x)
        .collect::<Vec<usize>>();
    peers.sort_unstable();
    peers
}

#[derive(Debug, Clone)]
struct Grid {
    data: Vec<PossibleValues>,
}

impl Grid {
    fn new() -> Self {
        let data = vec![DIGITS.to_string(); 81];
        Self { data }
    }

    fn at(&self, i: usize) -> &PossibleValues {
        &self.data[i]
    }

    fn set(&mut self, i: usize, value: String) {
        let (row, col) = dx(i);
        // TODO: eleminate all values except `value`
        // from other squares in the same row and col
        // check for contradictions
        let remaining = self.data[i].replace(&value, "");
        for d in remaining.chars() {
            // eliminate d from self.data[i]
            // (1) If a square s is reduced to one value d2, then eliminate d2 from the peers.
            // (2) If a unit u is reduced to only one place for a value d, then put it there.
        }
    }

    fn eliminate(&mut self, i: usize, value: char) {
        let found = self.data[i].chars().any(|c| c == value);
        if !found {
            return;
        } // already eliminated
        self.data[i] = self.data[i].replace(value, "");
        // (1) If a square is reduced to one value d2, then eliminate d2 from the peers.
        let len = self.data[i].len();
        if len == 0 {
            return;
        } // removed last value, this is a contradiction
        if len == 1 {
            // let d2 = self.data[i];
        }
    }

    fn get(&self, row: u8, col: u8) -> &PossibleValues {
        &self.data[ix(row, col)]
    }

    fn is_solved(&self) -> bool {
        self.data.iter().all(|values| values.len() == 1)
    }

    fn all_values(&self) -> std::slice::Iter<PossibleValues> {
        self.data.iter()
    }

    fn from_string(input: &String) -> Self {
        let input = input.replace("\n", "");
        // TODO: handle this error
        assert_eq!(input.len(), 81);

        let mut grid = Grid::new();
        for (i, input_value) in (0..81).zip(input.chars()) {
            if input_value != '0' {
                grid.data[i] = input_value.to_string();
            };
        }

        grid
    }
}

impl ToString for Grid {
    fn to_string(&self) -> String {
        let mut width = 1 + self.all_values().map(|values| values.len()).max().unwrap();
        if width == 10 {
            width = 2;
        }
        let col_hr: String = repeat("-").take(width + 1).collect();
        let sec_hr: String = repeat(col_hr).take(3).collect();
        let hr = repeat(sec_hr).take(3).collect::<Vec<String>>().join("+-");
        ROWS.iter()
            .map(|&row| {
                COLS.iter()
                    .map(|&col| {
                        let mut values = self.get(row, col).clone();
                        if values == DIGITS {
                            values = " ".to_string();
                        }
                        format!("{:width$}", values, width = width)
                    })
                    .collect::<Vec<_>>()
                    .chunks(3)
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|&chunk| chunk.join(" "))
                    .collect::<Vec<String>>()
                    .join(" | ")
            })
            .collect::<Vec<_>>()
            .chunks(3)
            .collect::<Vec<_>>()
            .join(&hr)
            .join("\n")
    }
}

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
    if grid.is_solved() {
        return Some(grid.clone());
    };
    let next = select_next(&grid);
    let possible_vals = grid.at(next);

    for val in possible_vals.chars() {
        let mut clone = grid.clone();
        clone.set(next, val.to_string());
        let result = search(&clone);
        if let Some(solution) = result {
            return Some(solution);
        }
    }

    None
}

fn select_next(grid: &Grid) -> usize {
    let (next, _) = grid
        .all_values()
        .enumerate()
        .map(|(i, vals)| (i, vals.len()))
        .filter(|&(_s, len)| len > 1)
        .min_by_key(|&(_s, len)| len)
        .unwrap();
    next
}

fn main() -> SResult<()> {
    let paths: Vec<String> = env::args().collect();
    run(&paths)?;
    Ok(())
}

fn run(paths: &Vec<String>) -> SResult<()> {
    let solutions: Vec<SResult<Option<Grid>>> = paths
        .iter()
        .map(|path| -> SResult<Option<Grid>> {
            let puzzle = load_puzzle(path)?;
            println!("{}", puzzle.grid.to_string());
            let solution = puzzle.solve();
            println!("{}", solution.unwrap().to_string());
            Ok(puzzle.solve())
        })
        .collect();

    Ok(())
}

fn load_puzzle<P: AsRef<Path>>(path: P) -> SResult<Puzzle> {
    let puzzle_str = fs::read_to_string(path)?;
    Ok(Puzzle::from_string(&puzzle_str))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peers_0() {
        assert_eq!(
            peers(0),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 18, 20, 27, 36, 45, 54, 63, 72]
        );
    }

    #[test]
    fn test_peers_1() {
        assert_eq!(
            peers(1),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 19, 20, 28, 37, 46, 55, 64, 73]
        );
    }

    #[test]
    fn test_ix() {
        assert_eq!(ix(0, 0), 0);
        assert_eq!(ix(1, 0), 1);
        assert_eq!(ix(0, 1), 9);
        assert_eq!(ix(8, 0), 8);
        assert_eq!(ix(0, 8), 72);
        assert_eq!(ix(8, 8), 80);
    }

    #[test]
    fn test_dx() {
        assert_eq!((0, 0), dx(0));
        assert_eq!((1, 0), dx(1));
        assert_eq!((0, 1), dx(9));
        assert_eq!((8, 0), dx(8));
        assert_eq!((0, 8), dx(72));
        assert_eq!((7, 8), dx(79));
        assert_eq!((8, 8), dx(80));
    }
}
