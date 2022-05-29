use std::{convert::TryFrom, fmt::{Display, Write}};

#[derive(Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Mine,
    Number(u8),
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Self::Empty => ' ',
            Self::Mine => '*',
            Self::Number(v) => std::char::from_digit(*v as u32, 10).unwrap(),
        })
    }
}

impl TryFrom<char> for Cell {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '*' => Ok(Cell::Mine),
            '.' => Ok(Cell::Empty),
            value => Err(format!("Unknown character {}", value)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Field {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

impl Field {
    fn read_sizes(value: &str) -> Result<(usize, usize), String> {
        let numbers = value
            .split_whitespace()
            .map(|v| v
                 .parse()
                 .map_err(|_| format!("Expected numbers on sizes but got an \"{}\".", v)))
            .collect::<Result<Vec<_>, _>>()?;

        match numbers[..] {
            [width, height] => Ok((width, height)),
            _ => Err(format!("Expected 2 arguments: width and height, found: {}.",
                             numbers.len())),
        }
    }

    pub fn solve(&self) -> Self {
        Self {
            width: self.width,
            height: self.height,
            cells: (0..self.cells.len())
                .map(|i| self.solve_cell(i))
                .collect(),
        }
    }

    fn solve_cell(&self, idx: usize) -> Cell {
        if self.cells[idx] == Cell::Mine {
            return Cell::Mine;
        }

        let (row, col) = (idx / self.width, idx % self.width);

        let (row_b, col_b) = (
            if row == 0 { 0 } else { row - 1 },
            if col == 0 { 0 } else { col - 1 },
        );

        let (row_e, col_e) = (
            if row == self.height - 1 { row } else { row + 1 },
            if col == self.width - 1 { col } else { col + 1 },
        );

        let (row, col) = (row - row_b, col - col_b);

        Cell::Number(
            self.cells
                .chunks(self.width)
                .skip(row_b).take(row_e - row_b + 1).enumerate()
                .map(|(i, cells)|
                     cells.iter()
                     .skip(col_b).take(col_e - col_b + 1).enumerate()
                     .filter(|(j, cell)|
                             matches!(cell, Cell::Mine if i != row || *j != col))
                     .count() as u8)
                .sum()
        )
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self.cells
                .chunks(self.width)
                .map(|v| v.iter()
                     .map(Cell::to_string)
                     .collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl TryFrom<Vec<String>> for Field {
    type Error = String;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let (width, height) = value
            .first()
            .map(|v| Self::read_sizes(v))
            .ok_or("Couldn't get field sizes.")??;

        if value.len() - 1 != height {
            return Err(format!("Lines count missmatches, expected: {}, found: {}.",
                               height, value.len() - 1));
        }

        let cells = value.iter()
            .skip(1)
            .enumerate()
            .map(|(i, v)| match v.len() == width {
                true => Ok(
                    v.chars()
                        .enumerate()
                        .map(|(j, ch)| ch
                             .try_into()
                             .map_err(|_| format!("Expected symbols '.' or '*', found: {} on line {} with index {}",
                                                  ch, i + 1, j + 1)))
                        .collect::<Vec<_>>()
                ),
                false => Err(format!("Expected width {}, found {} on line {}",
                                     width, v.len(), i + 1)),
            })
            .flat_map(|v| match v {
                Ok(v) => v,
                Err(e) => vec![Err(e)],
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            width,
            height,
            cells,
        })
    }
}


fn main() {
    let input = {
        let mut input = Vec::new();

        let mut line = String::new();

        while let Ok(size) = std::io::stdin().read_line(&mut line) {
            if size == 0 { break; }
            input.push(line.trim().to_owned());
            line.clear();
        }

        input
    };

    let field: Field = match input.try_into() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            return;
        },
    };

    let res = field.solve();

    println!("{}", res)
}


#[cfg(test)]
mod test {
    use crate::{Field, Cell};

    #[test]
    fn test_parser() {
        let input = "4 4
*...
....
.*..
...."
            .lines()
            .map(String::from)
            .collect::<Vec<_>>();

        let field: Result<crate::Field, _> = input.try_into();

        assert!(field.is_ok());

        let field = field.unwrap();

        let expected = Field {
            width: 4,
            height: 4,
            cells: vec![
                Cell::Mine, Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Mine, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty,
            ]
        };

        assert_eq!(field, expected);

        let expected = Field {
            width: 4,
            height: 4,
            cells: vec![
                Cell::Mine, Cell::Number(1), Cell::Number(0), Cell::Number(0),
                Cell::Number(2), Cell::Number(2), Cell::Number(1), Cell::Number(0),
                Cell::Number(1), Cell::Mine, Cell::Number(1), Cell::Number(0),
                Cell::Number(1), Cell::Number(1), Cell::Number(1), Cell::Number(0),
            ]
        };

        assert_eq!(field.solve(), expected);
    }

    #[test]
    fn test_solver() {
        let field = Field {
            width: 4,
            height: 4,
            cells: vec![
                Cell::Mine, Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Mine, Cell::Empty, Cell::Empty,
                Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty,
            ]
        };

        let expected = Field {
            width: 4,
            height: 4,
            cells: vec![
                Cell::Mine, Cell::Number(1), Cell::Number(0), Cell::Number(0),
                Cell::Number(2), Cell::Number(2), Cell::Number(1), Cell::Number(0),
                Cell::Number(1), Cell::Mine, Cell::Number(1), Cell::Number(0),
                Cell::Number(1), Cell::Number(1), Cell::Number(1), Cell::Number(0),
            ]
        };

        assert_eq!(field.solve(), expected);
    }

    
    #[test]
    fn test_broken_width() {
        let input = "5 4
*...
....
.*..
...."
            .lines()
            .map(String::from)
            .collect::<Vec<_>>();

        let field: Result<crate::Field, _> = input.try_into();

        assert!(field.is_err());
        assert!(field.unwrap_err().contains("Expected width"))
    }
    
    #[test]
    fn test_broken_heighth() {
        let input = "4 5
*...
....
.*..
...."
            .lines()
            .map(String::from)
            .collect::<Vec<_>>();

        let field: Result<crate::Field, _> = input.try_into();

        assert!(field.is_err());
        assert!(field.unwrap_err().contains("Lines count missmatches"))
    }

    #[test]
    fn test_broken_input_value() {
        let input = "ww5 4
*...
....
.*..
...."
            .lines()
            .map(String::from)
            .collect::<Vec<_>>();

        let field: Result<crate::Field, _> = input.try_into();

        assert!(field.is_err());
        assert!(field.unwrap_err().contains("Expected numbers on sizes but got an"))
    }
    
    #[test]
    fn test_broken_input_count() {
        let input = "4 4 666
*...
....
.*..
...."
            .lines()
            .map(String::from)
            .collect::<Vec<_>>();

        let field: Result<crate::Field, _> = input.try_into();

        assert!(field.is_err());
        assert!(field.unwrap_err().contains("Expected 2 arguments: width and height"))
    }

    #[test]
    fn test_broken_unknown_symbolt() {
        let input = "4 4
*.w.
....
.*..
...."
            .lines()
            .map(String::from)
            .collect::<Vec<_>>();

        let field: Result<crate::Field, _> = input.try_into();

        assert!(field.is_err());
        assert!(field.unwrap_err().contains("Expected symbols"))
    }
}
