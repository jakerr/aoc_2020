use std::ops::Index;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Tile {
    Tree,
    Snow,
}

pub struct Map {
    cells: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Index<(usize, usize)> for Map {
    type Output = Tile;
    fn index(&self, tuple: (usize, usize)) -> &Self::Output {
        let (x, y) = tuple;
        let x = x % self.width;
        let index = x + y * self.width;
        &self.cells[index]
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Map {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let mut width = 0usize;
    let mut tiles = vec![];
    for l in lines {
        let line = l.trim();
        if width == 0 {
            width = line.len();
        }
        for c in line.chars() {
            let tile = match c {
                '.' => Tile::Snow,
                '#' => Tile::Tree,
                _ => panic!("Invalid tile"),
            };
            tiles.push(tile);
        }
    }
    Map {
        cells: tiles,
        width,
        height,
    }
}

pub fn count_trees_on_slope(map: &Map, slope: (usize, usize)) -> u32 {
    let mut loc = (0, 0);
    let mut trees = 0u32;
    for y in 0..map.height {
        let mut tiles = vec![];
        for x in 0..map.width {
            let is_target = (loc.0 % map.width, loc.1) == (x, y);
            let e = match map[(x, y)] {
                Tile::Tree => {
                    if is_target {
                        trees += 1;
                        "X"
                    } else {
                        "#"
                    }
                }
                _ => {
                    if is_target {
                        "O"
                    } else {
                        "."
                    }
                }
            };
            tiles.push(e);
            if is_target {
                loc.0 += slope.0;
                loc.1 += slope.1;
            }
        }
        println!("{}", tiles.join(""))
    }
    println!("slope: {:?} trees: {:?}", slope, trees);
    trees
}

#[aoc(day3, part1)]
pub fn part1(map: &Map) -> u64 {
    count_trees_on_slope(map, (3, 1)).into()
}

#[aoc(day3, part2)]
pub fn part2(map: &Map) -> u64 {
    let slopes: Vec<u64> = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|s| count_trees_on_slope(map, *s).into())
        .collect();
    println!("slopes: {:?}", slopes);
    slopes.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generator() {
        let input = "..#\n#..";
        let map = input_generator(input);
        assert_eq!(map[(0, 0)], Tile::Snow);
        assert_eq!(map[(1, 0)], Tile::Snow);
        assert_eq!(map[(2, 0)], Tile::Tree);
        assert_eq!(map[(3, 0)], Tile::Snow);
        assert_eq!(map[(4, 0)], Tile::Snow);
        assert_eq!(map[(5, 0)], Tile::Tree);

        assert_eq!(map[(0, 1)], Tile::Tree);
        assert_eq!(map[(1, 1)], Tile::Snow);
        assert_eq!(map[(2, 1)], Tile::Snow);
        assert_eq!(map[(3, 1)], Tile::Tree);
        assert_eq!(map[(4, 1)], Tile::Snow);
        assert_eq!(map[(5, 1)], Tile::Snow);
    }

    #[test]
    fn part2_example_scores() {
        use indoc::indoc;
        let input = indoc! {"
            ..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#
        "};
        let map = input_generator(input);
        assert_eq!(map[(0, 0)], Tile::Snow);
        assert_eq!(map[(2, 0)], Tile::Tree);
        assert_eq!(count_trees_on_slope(&map, (1, 1)), 2);
        assert_eq!(count_trees_on_slope(&map, (3, 1)), 7);
        assert_eq!(count_trees_on_slope(&map, (5, 1)), 3);
        assert_eq!(count_trees_on_slope(&map, (7, 1)), 4);
        assert_eq!(count_trees_on_slope(&map, (1, 2)), 2);
        assert_eq!(part2(&map), 336);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn oob_index_should_panic() {
        let input = "..#\n#..";
        let map = input_generator(input);
        let _ = map[(0, 2)];
    }

    #[test]
    fn part1() {}
}
