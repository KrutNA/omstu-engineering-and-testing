#[derive(PartialEq, Eq)]
enum MazePart {
    Slash,
    Backslash,
}

impl TryFrom<char> for MazePart {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
	match value {
	    '\\' => Ok(Self::Backslash),
	    '/' => Ok(Self::Slash),
	    _ => Err(format!("Unknown symbol '{}'.", value)),
	}
    }
}

fn read_sizes() -> Option<(usize, usize)> {
    let mut line = String::new();

    if let Err(e) = std::io::stdin().read_line(&mut line) {
	eprintln!("Got an io error: {}.", e);
	return None;
    }

    let numbers = line.split_whitespace()
	.map(|v| v.parse::<usize>()
	     .map_err(|e| format!("Couldn't parse width/height: {}", e)))
	.collect::<Result<Vec<_>, _>>();
    let (width, height) = match numbers {
	Ok(v) if v.len() == 2 => (v[0], v[1]),
	Ok(v) => {
	    eprintln!("Expected 2 arguments, forund: {}", v.len());
	    return None;
	},
	Err(e) => {
	    eprint!("{}", e);
	    return None;
	},
    };

    if width == 0 && height == 0 { return None; }

    Some((width, height))
}

fn read_maze(width: usize, height: usize) -> Option<Vec<Vec<MazePart>>> {
    let mut line = String::new();
    let mut maze = vec![];

    for i in 0..height {
	if let Err(e) = std::io::stdin().read_line(&mut line) {
	    eprintln!("Got an IO error on line {}: {}.", i + 1, e);
	    return None;
	}
	line = line.trim().into();

	if line.len() != width as usize {
	    eprintln!("Expected width {}, found {}.", width, line.len());
	    return None;
	}
	
	let line = line.chars()
	    .map(|ch| ch.try_into())
	    .collect::<Result<Vec<MazePart>, _>>();

	match line {
	    Ok(line) => maze.push(line),
	    Err(e) => {
		eprintln!("{}", e);
		return None;
	    },
	}
    }

    Some(maze)
}

#[derive(PartialEq, Eq)]
enum Status {
    Empty,
    Slash,
    Backslash,
    Visited,
}

#[derive(PartialEq, Eq)]
enum Paths {
    LeftUp,
    Up,
    RightUp,
    Left,
    Right,
    LeftBottom,
    Down,
    RightBottom,
}

impl From<usize> for Paths {
    fn from(value: usize) -> Self {
	match value {
	    0 => Self::LeftUp,
	    1 => Self::Up,
	    2 => Self::RightUp,
	    3 => Self::Left,
	    4 => Self::Right,
	    5 => Self::LeftBottom,
	    6 => Self::Down,
	    7 => Self::RightBottom,
	    _ => unimplemented!(),
	}
    }
}

struct Maze {
    status: Vec<Vec<Status>>,
    width: usize,
    height: usize,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
	let (width, height) = (width * 2, height * 2);
	let capacity = width + height;
	let mut vec = Vec::with_capacity(capacity);

	for i in 0..vec.len() {
	    vec[i] = Vec::with_capacity(capacity);
	}
	
	Self {
	    status: vec,
	    width,
	    height,
	}
    }
}

static OFFSETS: [[isize; 2]; 8] = [
    [-1, -1], [-1, 0], [-1, 1], [0, -1],
    [0, 1], [1, -1], [1, 0], [1, 1]
];

fn in_range(line: isize, row: isize, width: usize, height: usize) -> bool {
    (0 <= line && (line as usize) < height) && (0 <= row && (row as usize) < width)
}

fn flood_fill(maze: &mut Maze, line: usize, row: usize, length: &mut usize) {
    *length += 1;

    maze.status[line][row] = Status::Visited;

    for offset in 0..OFFSETS.len() {
	let tline = line as isize + OFFSETS[offset][0];
	let trow = row as isize + OFFSETS[offset][1];

	if in_range(tline, trow, maze.width, maze.height) {
	    let (tline, trow) = (tline as usize, trow as usize);
	    if maze.status[tline][trow] == Status::Empty {
		if tline == line || trow == row {
		    flood_fill(maze, tline, trow, length);
		} else {
		    match offset.into() {
			Paths::LeftUp if maze.status[line][row - 1] != Status::Slash
			    => flood_fill(maze, tline, trow, length),
			Paths::RightBottom if maze.status[line][row + 1] != Status::Slash
			    => flood_fill(maze, tline, trow, length),
			Paths::LeftBottom if maze.status[line][row - 1] != Status::Backslash
			    => flood_fill(maze, tline, trow, length),
			Paths::RightUp if maze.status[line][row + 1] != Status::Backslash
			    => flood_fill(maze, tline, trow, length),
			_ => {},
		    }
		}
	    }
	}
    }
}

fn main() {
    loop {
	let (width, height) = if let Some((w, h)) = read_sizes() {
	    (w, h)
	} else { break; };

	let maze_op = if let Some(maze) = read_maze(width, height) {
	    maze
	} else { break; };
	
	let mut maze = Maze::new(width, height);

	for row_idx in 0..height {
	    for col_idx in 0..width {
		let maze_op = &maze_op[row_idx][col_idx];

		maze.status[row_idx * 2][col_idx * 2] = match maze_op {
		    MazePart::Backslash => Status::Backslash,
		    MazePart::Slash => Status::Empty,
		};
		maze.status[row_idx * 2][col_idx * 2 + 1] = match maze_op {
		    MazePart::Backslash => Status::Empty,
		    MazePart::Slash => Status::Slash,
		};
		maze.status[row_idx * 2 + 1][col_idx * 2] = match maze_op {
		    MazePart::Backslash => Status::Empty,
		    MazePart::Slash => Status::Slash,
		};
		maze.status[row_idx * 2 + 1][col_idx * 2 + 1] = match maze_op {
		    MazePart::Backslash => Status::Backslash,
		    MazePart::Slash => Status::Empty,
		};
	    }
	}

	let mut max = 0;
	let mut cycles = 0;

	for i in 0..maze.height {
	    for j in 0..maze.width {
		if maze.status[i][j] == Status::Empty {
		    cycles += 1;

		    let mut length = 0;
		    flood_fill(&mut maze, i, j, &mut length);

		    if max < length { max = length; }
		}
	    }
	}

	if max > 0 {
	    println!("Cycles: {}. Longest: {}.", cycles, max);
	} else {
	    println!("There are no cycles.")
	}
    }    
}
