pub fn count(lines: &[&str]) -> u32 {
    // unimplemented!("\nDetermine the count of rectangles in the ASCII diagram represented by the following lines:\n{:#?}\n.", lines);
    let rows = lines.len();
    if rows == 0 {
        return 0;
    }
    let cols = lines[0].len();

    let lines: Vec<Vec<char>> = lines.iter().map(|r| r.chars().collect()).collect();
    let mut recs = 0;
    for r in 0..(rows - 1) {
        for c in 0..(cols - 1) {
            if lines[r][c] == '+' {
                recs += visit(&lines, r, c, rows, cols);
            }
        }
    }
    recs as u32
}

fn visit(
    lines: &[Vec<char>],
    top_r: usize,
    left_c: usize,
    num_rows: usize,
    num_cols: usize,
) -> usize {
    let mut right_c = Vec::new();
    for c in (left_c + 1)..num_cols {
        match lines[top_r][c] {
            '-' => continue,
            '+' => right_c.push(c),
            _ => break,
        }
    }
    let mut bottom_r = Vec::new();
    for c in right_c.iter() {
        for r in (top_r + 1)..num_rows {
            match (lines[r][left_c], lines[r][*c]) {
                ('+', '+') => bottom_r.push(c),
                (a, b) if (a == '+' || a == '|') && (b == '+' || b == '|') => continue,
                _ => break,
            }
        }
    }
    bottom_r.len()
}
