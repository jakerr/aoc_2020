use itertools::Itertools;

#[derive(Debug)]
pub struct BoardingPass {
    row: u8,
    seat: u8,
}

impl BoardingPass {
    pub fn code_to_bin(code: &str) -> u8 {
        let mut bin_code = code.replace("F", "0");
        bin_code = bin_code.replace("B", "1");
        bin_code = bin_code.replace("R", "1");
        bin_code = bin_code.replace("L", "0");
        u8::from_str_radix(&bin_code, 2).unwrap()
    }

    pub fn new(code: &str) -> Self {
        let row_code = &code[0..7];
        let seat_code = &code[7..];
        BoardingPass {
            row: Self::code_to_bin(row_code),
            seat: Self::code_to_bin(seat_code),
        }
    }

    pub fn id(&self) -> u16 {
        (self.row as u16) * 8 + self.seat as u16
    }
}

#[aoc_generator(day5)]
pub fn generate(input: &str) -> Vec<BoardingPass> {
    input.lines().map(|s| BoardingPass::new(s)).collect()
}

#[aoc(day5, part1)]
pub fn part1(passes: &[BoardingPass]) -> u16 {
    passes.iter().map(|b| b.id()).max().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(passes: &[BoardingPass]) -> u16 {
    let mut last_id = 0u16;
    for id in passes.iter().map(|b| b.id()).sorted() {
        if last_id == 0 || last_id + 1 == id {
            last_id = id;
        } else {
            break;
        }
    }
    last_id + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example() {
        let pass = BoardingPass::new("BFFFBBFRRR");
        assert_eq!(pass.row, 70);
        assert_eq!(pass.seat, 7);
        assert_eq!(pass.id(), 567);

        let pass = BoardingPass::new("FFFBBBFRRR");
        assert_eq!(pass.row, 14);
        assert_eq!(pass.seat, 7);
        assert_eq!(pass.id(), 119);
    }
}
