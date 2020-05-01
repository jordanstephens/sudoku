use crate::util::*;
use std::iter::repeat;

type PossibleValues = String; // ex: "123456789"

#[derive(Debug, Clone)]
pub struct Grid {
    data: Vec<PossibleValues>,
}

impl Grid {
    pub fn new() -> Self {
        let data = vec![DIGITS.to_string(); 81];
        Self { data }
    }

    pub fn at(&self, i: usize) -> &PossibleValues {
        &self.data[i]
    }

    pub fn set(&mut self, i: usize, value: &str, depth: usize) -> bool {
        let remaining = self.at(i).replace(value, "");
        if remaining.is_empty() { return true }
        remaining.chars().all(|d| {
            self.eliminate(i, d, depth + 1)
        })
    }

    fn eliminate(&mut self, i: usize, value: char, depth: usize) -> bool {
        if !self.at(i).contains(value) {
            return true;
        } // already eliminated
        self.data[i] = self.at(i).replace(value, "");
        let len = self.at(i).len();
        if len == 0 {
            // removed last value, this is a contradiction
            return false;
        }
        if len == 1 {
            // all but one value have been eliminated from this square
            // so try to eliminate this value from this square's peers
            let d2 = self.at(i).chars().next().unwrap();
            let peers = peers(i);
            let result = peers.iter().all(|&j| {
                self.eliminate(j, d2, depth + 1)
            });

            if !result {
                return false;
            }
        }

        let units = units(i);
        // (2) If a unit u is reduced to only one place for value, then put it there.
        for u in units {
            // find all the other squares in this unit which still contain `value`
            let dplaces: Vec<&usize> = u.iter()
                .filter(|&&v| self.at(v).contains(value)).collect();
            let len = dplaces.len();
            if len == 0 {
                // contradiction
                return false;
            }
            if len == 1 {
                // if only one square in this unit has value, try to set the value there
                // (along with the associated eliminations)
                if !self.set(*dplaces[0], &value.to_string(), depth + 1) {
                    return false;
                }
            }
        }
        true
    }

    fn get(&self, row: u8, col: u8) -> &PossibleValues {
        &self.data[ix(row, col)]
    }

    pub fn is_solved(&self) -> bool {
        self.data.iter().all(|values| values.len() == 1)
    }

    pub fn all_values(&self) -> std::slice::Iter<PossibleValues> {
        self.data.iter()
    }

    pub fn from_string(input: &String) -> Self {
        let input = input.replace("\n", "");
        // TODO: handle this error
        assert_eq!(input.len(), 81);

        let mut grid = Grid::new();
        for (i, input_value) in (0..81).zip(input.chars()) {
            if input_value != '0' {
                let succeeded = grid.set(i, &input_value.to_string(), 0);
                if !succeeded {
                    panic!("invalid puzzle file");
                }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let g = Grid::new();
        assert_eq!(g.data, vec!["123456789"; 81]);
    }

    #[test]
    fn test_single_set() {
        let mut g = Grid::new();
        let succeeded = g.set(0, &"1".to_string(), 0);
        assert!(succeeded);
        assert_eq!(g.data[0], "1");
        for i in peers(0) {
            assert!(!g.data[i].contains("1"));
        }
    }

    #[test]
    fn test_successive_set() {
        let mut g = Grid::new();
        let succeeded1 = g.set(0, &"1".to_string(), 0);
        assert!(succeeded1);
        let succeeded2 = g.set(1, &"2".to_string(), 0);
        assert!(succeeded2);
        assert_eq!(g.data[1], "2");
        for i in peers(1) {
            assert!(!g.data[i].contains("2"));
        }
    }

    #[test]
    fn test_successive_set_2() {
        let mut g = Grid::new();
        let succeeded1 = g.set(0, &"4".to_string(), 0);
        assert!(succeeded1);
        let succeeded2 = g.set(6, &"8".to_string(), 0);
        assert!(succeeded2);
    }

    #[test]
    fn test_incompatible_set() {
        let mut g = Grid::new();
        let succeeded1 = g.set(0, &"1".to_string(), 0);
        assert!(succeeded1);
        let succeeded2 = g.set(1, &"1".to_string(), 0);
        assert!(!succeeded2);
    }
}
