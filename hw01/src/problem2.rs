/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    assert_eq!(mat1[0].len(), mat2.len());

    let mut final: Matrix = Vec::with_capacity(mat2.len());

    // A: [[1,2,3],[4,5,6]] - 2x3
    // B: [[1,2],[3,4],[5,6]] - 3x2
    // A*B = [[a11*b11 + a12*b21 + a13*b31, a11*b12 + a12*b22 + a13*b32], [a21*b11 + a22*b21 + a23*b31, a21*b12 + a22*b22 + a23*b32]]


    final[0][0] = mat1[0][0]*mat2[0][0] + mat1[0][1]*mat2[1][0] + mat1[0][2]*mat2[2][0]
    final[0][1] =
    final[1][0]
    final[1][1]
    for i in 0..mat1.len() {
        final[i] = Vec::with_capacity(mat2[0].len());
        for j in 0..mat2[0].len() {
            final[i][j] = mat1[i][j]
        }
        for j in 0..mat2.len() {
            mat1[i][j] * mat2[j][i]
        }
    }

    for mat1_row in mat1 {
        for mat1_row_col in mat1_row {
            mat1_row_col *
        }
    }

    vec![vec![1f32]]
}
