/// 核心在这里，把遍历的点作为坐标原点，
/// 遍历8个方向的地雷数量
static NEIGBOURHOOD_OFFSETS: &[(i32, i32)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();
    (0..height)
        .map(|y| {
            let width = minefield[y].len();
            (0..width)
                .map(|x| {
                    if minefield[y].as_bytes()[x] == b'*' {
                        '*'
                    } else {
                        // 如果不是地雷，遍历8个方向的地雷数
                        match NEIGBOURHOOD_OFFSETS
                            .iter()
                            // 移动坐标，把遍历的点作为坐标原点
                            .map(|&(ox, oy)| (x as i32 + ox, y as i32 + oy))
                            // 在坐标范围内，遍历8个方向
                            .filter(|&(x, y)| {
                                (0 <= x && x < width.try_into().unwrap())
                                    && (0 <= y && y < height.try_into().unwrap())
                            })
                            // 发现地雷，进行计数
                            .filter(|&(x, y)| minefield[y as usize].as_bytes()[x as usize] == b'*')
                            .count()
                        {
                            0 => ' ',
                            n => (n as u8 + b'0') as char,
                        }
                    }
                })
                .collect()
        })
        .collect()
}
