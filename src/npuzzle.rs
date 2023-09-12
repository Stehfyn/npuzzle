use rand::Rng;
const N_MIN: usize = 2;

#[derive(Clone)]
pub enum TileType {
    InPlay,
    Missing,
}

#[derive(Clone)]
pub struct Tile {
    pub tile_type: TileType,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            tile_type: TileType::InPlay,
        }
    }
}

pub struct NBoard {
    n: usize,
    board: Vec<Tile>,
    initial_board: Vec<Tile>,
}

impl NBoard {
    pub fn new(n: usize) -> Self {
        let mut initial_board = Vec::default();
        Self::initialize_board(&mut initial_board, n);
        let board = initial_board.clone();

        Self {
            n: n,
            board: board,
            initial_board,
        }
    }
}

impl Default for NBoard {
    fn default() -> Self {
        Self::new(N_MIN)
    }
}

pub enum Punchout {
    Random,
    Index(usize),
}

impl NBoard {
    pub fn generate(&mut self) {
        while Self::is_solvable(&self.board) {
            Self::reset(&mut self.board, &self.initial_board);
            Self::punchout(&mut self.board, Punchout::Random);
            Self::generate_puzzle(&mut self.board);
        }
    }

    pub fn reset(dst: &mut Vec<Tile>, src: &Vec<Tile>) {
        *dst = src.clone();
    }

    fn punchout(board: &mut Vec<Tile>, punch: Punchout) {
        match punch {
            Punchout::Random => {
                let i: usize = rand::thread_rng().gen_range(0..board.len());
                board.get_mut(i).unwrap().tile_type = TileType::Missing;
            }
            Punchout::Index(i) => {
                board.get_mut(i).unwrap().tile_type = TileType::Missing;
            }
        }
    }

    fn generate_puzzle(board: &mut Vec<Tile>) {
        
    }

    fn is_solvable(board: &Vec<Tile>) -> bool {
        true
    }

    fn initialize_board(board: &mut Vec<Tile>, n: usize) {
        let tile_count = n * n;
        for i in 0..tile_count {
            board.insert(i, Tile::default());
        }
    }
}
