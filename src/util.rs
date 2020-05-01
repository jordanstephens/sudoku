use std::collections::HashSet;

const RANGE: [u8; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
pub const ROWS: [u8; 9] = RANGE;
pub const COLS: [u8; 9] = RANGE;
pub const DIGITS: &'static str = r#"123456789"#;

pub fn ix(row: u8, col: u8) -> usize {
    assert!(row < 9);
    assert!(col < 9);
    (row * 9 + col) as usize
}

pub fn dx(i: usize) -> (u8, u8) {
    assert!(i < 81);
    let i = i as u8;
    let row = i / 9;
    let col = i % 9;
    (row, col)
}

pub fn col_cells(i: usize) -> Vec<usize> {
    let (_row, col) = dx(i);
    ROWS.clone().iter().map(|&row| ix(row, col)).collect()
}

pub fn row_cells(i: usize) -> Vec<usize> {
    let (row, _col) = dx(i);
    COLS.clone().iter().map(|&col| ix(row, col)).collect()
}

pub fn box_cells(i: usize) -> Vec<usize> {
    let (row, col) = dx(i);
    let box_row = (row as f32 / 9.0 * 3.0) as u8;
    let box_col = (col as f32 / 9.0 * 3.0) as u8;
    let box_row_start = box_row * 3;
    let box_col_start = box_col * 3;
    let box_row_end = box_row_start + 3;
    let box_col_end = box_col_start + 3;
    let row_range: Vec<u8> = (box_row_start..box_row_end).collect();
    let col_range: Vec<u8> = (box_col_start..box_col_end).collect();
    row_range.iter().flat_map(|&row| {
        col_range.iter().map(move |&col| ix(row, col))
    }).collect()
}

pub fn units(i: usize) -> Vec<Vec<usize>> {
    vec![row_cells(i), col_cells(i), box_cells(i)]
}

pub fn peers(i: usize) -> Vec<usize> {
    let units = units(i);
    let mut peers = units
        .concat()
        .iter()
        .fold(HashSet::new(), |mut acc, &i| {
            acc.insert(i);
            acc
        })
        .iter()
        .map(|x| *x)
        .filter(|&x| x != i)
        .collect::<Vec<usize>>();
    peers.sort_unstable();
    peers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peers_0() {
        assert_eq!(
            peers(0),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 18, 19, 20, 27, 36, 45, 54, 63, 72]
        );
    }

    #[test]
    fn test_peers_1() {
        assert_eq!(
            peers(1),
            vec![0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 18, 19, 20, 28, 37, 46, 55, 64, 73]
        );
    }

    #[test]
    fn test_peers_6() {
        assert_eq!(
            peers(6),
            vec![0, 1, 2, 3, 4, 5, 7, 8, 15, 16, 17, 24, 25, 26, 33, 42, 51, 60, 69, 78]
        );
    }

    #[test]
    fn test_row_cells_0() {
        assert_eq!(
            row_cells(0),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8]
        );
    }

    #[test]
    fn test_row_cells_1() {
        assert_eq!(
            row_cells(1),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8]
        );
    }

    #[test]
    fn test_row_cells_9() {
        assert_eq!(
            row_cells(9),
            vec![9, 10, 11, 12, 13, 14, 15, 16, 17]
        );
    }

    #[test]
    fn test_col_cells_0() {
        assert_eq!(
            col_cells(0),
            vec![0, 9, 18, 27, 36, 45, 54, 63, 72]
        );
    }

    #[test]
    fn test_col_cells_1() {
        assert_eq!(
            col_cells(1),
            vec![1, 10, 19, 28, 37, 46, 55, 64, 73]
        );
    }

    #[test]
    fn test_col_cells_6() {
        assert_eq!(
            col_cells(6),
            vec![6, 15, 24, 33, 42, 51, 60, 69, 78]
        );
    }

    #[test]
    fn test_box_cells_0() {
        assert_eq!(
            box_cells(0),
            vec![0, 1, 2, 9, 10, 11, 18, 19, 20]
        );
    }

    #[test]
    fn test_box_cells_6() {
        assert_eq!(
            box_cells(6),
            vec![6, 7, 8, 15, 16, 17, 24, 25, 26]
        );
    }

    #[test]
    fn test_ix() {
        assert_eq!(ix(0, 0), 0);
        assert_eq!(ix(0, 1), 1);
        assert_eq!(ix(1, 0), 9);
        assert_eq!(ix(0, 8), 8);
        assert_eq!(ix(8, 0), 72);
        assert_eq!(ix(8, 8), 80);
    }

    #[test]
    fn test_dx() {
        assert_eq!((0, 0), dx(0));
        assert_eq!((0, 1), dx(1));
        assert_eq!((1, 0), dx(9));
        assert_eq!((0, 8), dx(8));
        assert_eq!((8, 0), dx(72));
        assert_eq!((8, 7), dx(79));
        assert_eq!((8, 8), dx(80));
    }
}
