#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
    Empty,
    Mine,
    Number(u32),
}

fn to_cells(s: &str) -> Vec<Cell> {
    s.chars()
        .map(|c: char| {
            match c {
                '*' => Cell::Mine,
                ' ' => Cell::Empty,
                x => Cell::Number(x.to_digit(10).unwrap()),
            }
        })
        .collect()
}

fn from_cells(v: Vec<Cell>) -> String {
    v.iter()
        .map(|cell| {
            match *cell {
                Cell::Mine => '*',
                Cell::Empty => ' ',
                Cell::Number(x) => (x as u8 + b'0') as char,
            }
        })
        .collect()
}

fn permutations(x: isize, y: isize, x_limit: isize, y_limit: isize) -> Vec<(usize, usize)> {
    [(x - 1, y - 1), (x, y - 1), (x + 1, y - 1), (x + 1, y), (x + 1, y + 1), (x, y + 1), (x - 1, y + 1), (x - 1, y)]
        .iter()
        .filter(|&&(a, b)| a >= 0 && b >= 0 && a < x_limit && b < y_limit).map(|&pair| pair)
        .map(|(a, b)| (a as usize, b as usize))
        .collect()
}

pub fn annotate(field: &[&str]) -> Vec<String> {
    let mut res: Vec<Vec<Cell>> = field.iter().map(|l| to_cells(l)).collect();
    let mine_field: Vec<Vec<Cell>> = field.iter().map(|l| to_cells(l)).collect();

    for (y, row) in mine_field.iter().enumerate() {
        for (x, _) in row.iter().enumerate().filter(|&(_, cell)| *cell == Cell::Empty) {
            for (x_neighbor, y_neighbor) in permutations(x as isize, y as isize, row.len() as isize, mine_field.len() as isize) {
                match mine_field[y_neighbor][x_neighbor] {
                    Cell::Mine => {
                        match res[y][x] {
                            Cell::Number(count) => res[y as usize][x as usize] = Cell::Number(count + 1),
                            Cell::Empty => res[y as usize][x as usize] = Cell::Number(1),
                            _ => ()
                        }
                    },
                    _ => ()
                }
            }
        }
    }

    res.iter().map(|cells| from_cells(cells.to_vec())).collect()
}