use rand::Rng;
const N_MIN: usize = 2;
use log::{debug, error, info};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GenerationMetric {
    MaxManhattanDistance,
    MaxEuclideanDistance,
    MaxTilesOut,
    Random(usize),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TileType {
    InPlay,
    Missing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Tile {
    index: usize,
    pub tile_type: TileType,
}

impl Default for Tile {
    fn default() -> Self {
        Self::new(0, TileType::InPlay)
    }
}

impl Tile {
    pub fn new(index: usize, tile_type: TileType) -> Self {
        Self {
            index: index,
            tile_type: tile_type,
        }
    }
}

pub struct NBoard {
    n: usize,
    board: Vec<Tile>,
    initial_board: Vec<Tile>,
    missing_index: usize,
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
            missing_index: 9999,
        }
    }
}

impl Default for NBoard {
    fn default() -> Self {
        Self::new(N_MIN)
    }
}

enum Punchout {
    Random,
    Index(usize),
}

impl NBoard {
    pub fn generate(&mut self) {
        Self::reset(&mut self.board, &mut self.initial_board);
        self.missing_index = Self::punchout(&mut self.board, Punchout::Random);
        self.missing_index = Self::generate_puzzle(&mut self.board, GenerationMetric::Random(10));

        /*
        while {
            Self::reset(&mut self.board, &mut self.initial_board);
            Self::punchout(&mut self.board, Punchout::Random);
            Self::generate_puzzle(&mut self.board, GenerationMetric::Random(100));
            //Self::is_solvable(&mut self.board)
            false
        } {
            break;
        }*/
    }

    pub fn reset(dst: &mut Vec<Tile>, src: &Vec<Tile>) {
        *dst = src.clone();
    }

    pub fn index_at(&self, i: usize) -> usize {
        if let Some(tile) = self.board.get(i) {
            tile.index
        } else {
            panic!(
                "index_at: Out of bounds! board_size: {}, i: {}",
                self.board.len(),
                i
            );
        }
    }

    fn punchout(board: &mut Vec<Tile>, punch: Punchout) -> usize {
        match punch {
            Punchout::Random => {
                let i: usize = rand::thread_rng().gen_range(0..board.len());
                board.get_mut(i).unwrap().tile_type = TileType::Missing;
                i
            }
            Punchout::Index(i) => {
                board.get_mut(i).unwrap().tile_type = TileType::Missing;
                i
            }
        }
    }

    pub fn check_win(&self) -> bool {
        for i in 0..self.board.len() {
            if let Some(tile) = self.board.get(i) {
                if tile.index != i {
                    break;
                } else if i == (self.board.len() - 1) {
                    return true;
                }
            }
        }
        false
    }

    pub fn swap(&mut self, with: usize) -> usize {
        self.missing_index = Self::swap_with(self.missing_index, with, &mut self.board);
        self.missing_index
    }

    fn swap_with(missing_index: usize, with: usize, board: &mut Vec<Tile>) -> usize {
        board.swap(missing_index, with);
        with
    }

    fn generate_puzzle(board: &mut Vec<Tile>, metric: GenerationMetric) -> usize {
        match metric {
            GenerationMetric::MaxTilesOut => 0,
            GenerationMetric::MaxManhattanDistance => 0,
            GenerationMetric::MaxEuclideanDistance => 0,
            GenerationMetric::Random(move_count) => {
                let mut missing_index = Self::find_index_of_missing(board);
                for _ in 0..move_count {
                    let available_to_swap =
                        Self::get_available_to_swap(missing_index, Self::get_n_from_board(board));

                    let with = available_to_swap
                        .get(rand::thread_rng().gen_range(0..available_to_swap.len()))
                        .unwrap()
                        .clone();

                    debug!("{}", format!("random_tile_index: {with}"));

                    missing_index = Self::swap_with(missing_index, with, board);
                }
                missing_index
            }
        }
    }

    pub fn get_swappable(&self) -> Vec<usize> {
        Self::get_available_to_swap(self.missing_index, self.n)
    }

    fn get_available_to_swap(missing_index: usize, n: usize) -> Vec<usize> {
        let mut available_to_swap = Vec::default();

        // Left
        if missing_index.rem_euclid(n) > 0 {
            available_to_swap.insert(available_to_swap.len(), missing_index - 1);
        }

        // Right
        if missing_index.rem_euclid(n) < (n - 1) {
            available_to_swap.insert(available_to_swap.len(), missing_index + 1);
        }

        // Below
        if (missing_index / n) > 0 {
            available_to_swap.insert(available_to_swap.len(), missing_index - n);
        }

        // Above
        if (missing_index / n) < (n - 1) {
            available_to_swap.insert(available_to_swap.len(), missing_index + n);
        }

        available_to_swap
    }

    pub fn missing_index(&self) -> usize {
        self.missing_index
    }

    fn find_index_of_missing(board: &Vec<Tile>) -> usize {
        for i in 0..board.len() {
            if let Some(tile) = board.get(i) {
                if tile.tile_type == TileType::Missing {
                    debug!("missing_index: {}", i);
                    return i;
                }
            }
        }
        panic!("All tiles are in play!");
    }

    fn get_n_from_board(board: &Vec<Tile>) -> usize {
        (board.len() as f64).sqrt() as usize
    }

    fn neither_are_missing(t1: &Tile, t2: &Tile) -> bool {
        (t1.tile_type != TileType::Missing) && (t2.tile_type != TileType::Missing)
    }

    fn count_inversions(board: &Vec<Tile>) -> usize {
        let mut count = 0;
        let n = Self::get_n_from_board(board);

        for i in 0..n {
            for j in 1..n {
                let t1 = board[i];
                let t2 = board[j];
                if Self::neither_are_missing(&t1, &t2) && (t1.index > t2.index) {
                    count += 1
                }
            }
        }
        count
    }

    fn get_row_from_index(index: usize, n: usize) -> usize {
        index / n
    }

    //refactor for better complexity
    fn is_solvable(board: &Vec<Tile>) -> bool {
        let inversion_count = Self::count_inversions(board);
        let n = Self::get_n_from_board(board);

        if !is_even(n) {
            return is_even(inversion_count);
        } else {
            let missing_index = Self::find_index_of_missing(board);
            let missing_index_row = Self::get_row_from_index(missing_index, n);

            return (is_even(inversion_count) && !is_even(missing_index_row))
                || (!is_even(inversion_count) && is_even(missing_index_row));
        }
    }

    fn initialize_board(board: &mut Vec<Tile>, n: usize) {
        let tile_count = n * n;
        for i in 0..tile_count {
            board.insert(i, Tile::new(i, TileType::InPlay));
        }
    }
}

fn is_even(x: usize) -> bool {
    x.rem_euclid(2) == 0
}
