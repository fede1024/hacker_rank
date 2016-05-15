use std::io;
use std::io::prelude::*;
use std::cmp;

struct Matrix {
    data: Vec<Vec<i32>>,
    height: usize,
    width: usize,
}

impl Matrix {
    fn new(height: usize, width: usize) -> Matrix {
        Matrix{height: height, width: width, data: vec![vec![0; width]; height]}
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width { print!("{} ", self.data[y][x]); }
            println!("");
        }
    }
}

fn read_matrix<T: Iterator<Item=String>>(lines: &mut T) -> (Matrix, usize) {
    let line = lines.next().unwrap();
    let size = line.split(" ").collect::<Vec<_>>();
    let height = size[0].parse::<usize>().unwrap();
    let width = size[1].parse::<usize>().unwrap();
    let moves = size[2].parse::<usize>().unwrap();
    let mut matrix = Matrix::new(height, width);
    for y in 0..height {
        let line = lines.next().unwrap();
        let nums = line.split(" ").collect::<Vec<_>>();
        for x in 0..width {
            matrix.data[y][x] = nums[x].parse::<i32>().unwrap();
        }
    }
    (matrix, moves)
}

fn create_coords(start_y: usize, end_y: usize, start_x: usize, end_x: usize) -> Vec<(usize, usize)> {
    let mut coords = Vec::new();

    coords.extend((start_y..end_y).map(|y| (y, start_x)));
    coords.extend((start_x..end_x).map(|x| (end_y, x)));
    coords.extend((start_y..(end_y+1)).rev().map(|y| (y, end_x)));
    coords.extend(((start_x+1)..end_x).rev().map(|x| (start_y, x)));
    coords
}

fn rotate(matrix: &Matrix, moves: usize) -> Matrix {
    let mut out_matrix = Matrix::new(matrix.height, matrix.width);
    for n in 0..(cmp::min(matrix.height, matrix.width)/2) {
        let coords = create_coords(n, matrix.height-n-1, n, matrix.width-n-1);
        for (n, &(y, x)) in coords.iter().enumerate() {
            let (dy, dx) = coords[(n + moves) % coords.len()];
            out_matrix.data[dy][dx] = matrix.data[y][x];
        }
    }
    out_matrix
}

fn main() {
    let stdin = io::stdin();
    let mut lines_iter = stdin.lock().lines().map(|x| x.unwrap());
    let (matrix, moves) = read_matrix(&mut lines_iter);
    rotate(&matrix, moves).print();
}
