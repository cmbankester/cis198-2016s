/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    assert_eq!(mat1[0].len(), mat2.len());
    let mat1_num_rows = mat1.len();
    let mat1_num_cols = mat1[0].len();
    let mat2_num_cols = mat2[0].len();

    let mut dot_product: Matrix = Vec::with_capacity(mat1_num_cols);

    // A: [[1,2,3],[4,5,6]] - 2x3
    // B: [[1,2,2,2],[3,4,2,2],[5,6,2,2]] - 3x4

    // (a00*b00 + a01*b10 + a02*b20), (a00*b01 + a01*b11 + a02*b21), (a00*b02 + a01*b12 + a02*b22), (a00*b03 + a01*b13 + a02*b23)
    // (a10*b00 + a11*b10 + a12*b20), (a10*b01 + a11*b11 + a12*b21), (a10*b02 + a11*b12 + a12*b22), (a10*b03 + a11*b13 + a12*b23)

    for i in 0..mat1_num_rows {
        dot_product.push(Vec::with_capacity(mat2_num_cols));
        for j in 0..mat2_num_cols {
            dot_product[i].push(0f32);
            for k in 0..mat1_num_cols {
                dot_product[i][j] += mat1[i][k] * mat2[k][j];
            }
        }
    }

    dot_product
}
