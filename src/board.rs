extern crate nalgebra as na;

use na::{Matrix, MatrixArray, U3};
use square::Square;

/// 3x3 board
pub type Board = Matrix<Square, U3, U3, MatrixArray<Square, U3, U3>>;
