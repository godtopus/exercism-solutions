pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows: Vec<Vec<u32>> = Vec::with_capacity((row_count + 1) as usize);

        for _ in 1..(row_count + 1) {
            match rows.is_empty() {
                true => rows.push(vec![1]),
                false => {
                    let mut next_row = vec![1];
                    for chunk in rows.last().unwrap().windows(2) {
                        next_row.push(chunk.iter().sum::<u32>());
                    }
                    next_row.push(1);
                    rows.push(next_row.clone());
                }
            }
        }

        PascalsTriangle {
            rows: rows
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
