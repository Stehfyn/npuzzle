/**
 * @file npuzzle.rs
 *
 * @brief This is the npuzzle module which implements the logic for an NxN Board with it's accompanying,
 * uninformed and heuristic-based search algorithms. (DFS and A*)
 *
 * Generation technique and is_solvable is adapted from:
 * https://github.com/tnicolas42/n-puzzle/blob/master/generator.py
 *
 * Algorithms adapted from:
 * https://github.com/tnicolas42/n-puzzle/blob/master/srcs/algo.py
 *
 * Heuristics adapted from:
 * https://github.com/tnicolas42/n-puzzle/blob/master/srcs/heuristics.py
 *
 * @author Stephen Foster
 * Contact: stephenfoster@nevada.unr.edu
 *
 */
use rand::Rng;
const N_MIN: usize = 2;
use log::{debug, error, info};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

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

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.index)
    }
}

#[derive(Eq)]
struct State {
    cost: usize,
    board: NBoard,
    steps: Vec<usize>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
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

use std::fmt;

impl std::fmt::Display for NBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = "".to_owned();
        for i in 0..self.n {
            for j in 0..self.n {
                s += &format!("{}", self.board.get((i * self.n) + j).unwrap())[..];
                s += " ";
            }
            s += "\n";
        }
        write!(f, "{}", s)
    }
}

enum Punchout {
    Random,
    Index(usize),
}

impl NBoard {
    pub fn generate(&mut self) {
        while {
            Self::reset(&mut self.board, &mut self.initial_board);
            self.missing_index = Self::punchout(&mut self.board, Punchout::Random);
            self.missing_index = Self::generate_puzzle(
                &mut self.board,
                self.missing_index,
                GenerationMetric::Random(100),
            );
            !self.check_win()
        } {
            break;
        }
    }

    pub fn set_board(&mut self, new_board: Vec<Tile>) {
        self.board = new_board;
    }

    pub fn set_mi(&mut self, mi: usize) {
        self.missing_index = mi;
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

    fn generate_puzzle(board: &mut Vec<Tile>, missing: usize, metric: GenerationMetric) -> usize {
        match metric {
            GenerationMetric::MaxTilesOut => 0,
            GenerationMetric::MaxManhattanDistance => 0,
            GenerationMetric::MaxEuclideanDistance => 0,
            GenerationMetric::Random(move_count) => {
                let mut missing_index = missing;
                for _ in 0..move_count {
                    let available_to_swap =
                        Self::get_available_to_swap(missing_index, Self::get_n_from_board(board));

                    let with = available_to_swap
                        .get(rand::thread_rng().gen_range(0..available_to_swap.len()))
                        .unwrap()
                        .clone();

                    missing_index = Self::swap_with(missing_index, with, board);
                }
                missing_index
            }
        }
    }

    pub fn get_swappable(&self) -> Vec<usize> {
        Self::get_available_to_swap(self.missing_index, self.n)
    }

    pub fn to_string_representation(&self) -> String {
        let mut s = String::new();
        for tile in &self.board {
            s.push_str(&format!("{:?}", tile));
        }
        s
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

    pub fn dfs_solve(&mut self) -> Option<Vec<NBoard>> {
        let mut visited: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut path = Vec::new();
        if self._dfs_solve(&mut visited, &mut path) {
            Some(path)
        } else {
            None
        }
    }

    fn _dfs_solve(
        &mut self,
        visited: &mut std::collections::HashSet<String>,
        path: &mut Vec<NBoard>,
    ) -> bool {
        if self.check_win() {
            path.push(self.clone());
            return true;
        }

        let current_state = self.to_string_representation();
        if visited.contains(&current_state) {
            return false;
        }
        visited.insert(current_state.clone());

        for &next_index in &Self::get_available_to_swap(self.missing_index, self.n) {
            self.swap(next_index);
            if self._dfs_solve(visited, path) {
                path.push(self.clone());
                return true;
            }
            self.swap(next_index); // backtrack
        }
        false
    }

    pub fn a_star_solve(&self) -> Option<Vec<usize>> {
        let mut visited: HashSet<String> = HashSet::new();
        let mut heap = BinaryHeap::new();

        heap.push(State {
            cost: 0,
            board: self.clone(),
            steps: Vec::new(),
        });

        while let Some(State { cost, board, steps }) = heap.pop() {
            if board.check_win() {
                return Some(steps);
            }

            let state_str = board.to_string();
            if visited.contains(&state_str) {
                continue;
            }
            visited.insert(state_str);

            for swappable_index in board.get_swappable() {
                let mut new_board = board.clone();
                new_board.swap(swappable_index);

                let mut new_steps = steps.clone();
                new_steps.push(swappable_index);

                heap.push(State {
                    cost: new_steps.len() + new_board.manhattan_distance(),
                    board: new_board,
                    steps: new_steps,
                });
            }
        }
        None
    }

    pub fn get_missing_index(&self) -> usize {
        self.missing_index
    }

    fn get_n_from_board(board: &Vec<Tile>) -> usize {
        (board.len() as f64).sqrt() as usize
    }

    fn neither_are_missing(t1: &Tile, t2: &Tile) -> bool {
        (t1.tile_type != TileType::Missing) && (t2.tile_type != TileType::Missing)
    }

    fn manhattan_distance(&self) -> usize {
        let mut distance = 0;
        for (i, tile) in self.board.iter().enumerate() {
            if tile.tile_type == TileType::Missing {
                continue;
            }
            let final_x = tile.index % self.n;
            let final_y = tile.index / self.n;
            let current_x = i % self.n;
            let current_y = i / self.n;
            distance += (final_x as isize - current_x as isize).abs()
                + (final_y as isize - current_y as isize).abs();
        }
        distance as usize
    }

    fn count_inversions(board: &Vec<Tile>) -> usize {
        let mut count = 0;
        let n = board.len(); // Assuming board is a flat array representing N x N board

        for i in 0..n {
            for j in (i + 1)..n {
                let t1 = &board[i];
                let t2 = &board[j];
                if Self::neither_are_missing(t1, t2) && (t1.index > t2.index) {
                    count += 1;
                }
            }
        }
        count
    }

    fn get_row_from_index(index: usize, n: usize) -> usize {
        index / n
    }

    pub fn solvable(&self) -> bool {
        if self.check_win() {
            return true;
        }
        if self.n == 2 || self.n == 3 {
            let mut check = self.clone();
            if let Some(solution) = check.a_star_solve() {
                return true;
            } else {
                return false;
            }
        }

        Self::is_solvable(&self.board, self.missing_index, self.n)
    }

    fn is_solvable(board: &Vec<Tile>, missing_index: usize, n: usize) -> bool {
        let inversion_count = Self::count_inversions(board);
        if !is_even(n) {
            return is_even(inversion_count);
        } else {
            let missing_index_row = n - Self::get_row_from_index(missing_index, n);
            return is_even(inversion_count) == is_even(missing_index_row);
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
    (x as i32).rem_euclid(2) == 0
}
