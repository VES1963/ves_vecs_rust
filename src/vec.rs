mod matrix {

    /// This [Matrix] struct is used to represent a 2d matrix for math operations
    #[derive(Debug, Copy, Clone)]
    pub struct Matrix<const X: usize, const Y: usize> {
        pub values: [[f64; Y]; X]
    }

    impl<const Y: usize, const X: usize> Matrix<Y, X> {

        /// Creates a new [Matrix] from a 2d array
        pub fn new(mat: [[f64; X]; Y]) -> Self {
            Self { values: mat }
        }

        /// Transposes the [Matrix] so that it is flipped around the diagonal
        pub fn transpose(&self) -> Matrix<X, Y> {
            let mut new_vec = Matrix::<X, Y>::default();

            for x in 0..X {
                for y in 0..Y {
                    new_vec[x][y] = self[y][x]
                }
            }

            new_vec
        }
    }

    impl<const N: usize> Matrix<N, N> {

        /// Creates a square scalar [Matrix] that is used to scale matrices
        pub fn scalar(s: f64) -> Self {
            let mut new_mat = Self::default();

            for i in 0..N {
                new_mat[i][i] = s
            }

            new_mat
        }

        /// Creates an identity [Matrix] that when multiplied with another matrix returns the same matrix
        pub fn identity() -> Self {
            Self::scalar(1.)
        }

        /// Creates a square rotation [Matrix] typically used to rotate [Vector]s
        pub fn rotation_d(c1: usize, c2: usize, degrees: f64) -> Self {
            let mut new_mat = Self::identity();

            new_mat[c1][c1] =  f64::cos(degrees.to_radians());
            new_mat[c1][c2] =  f64::sin(degrees.to_radians());
            new_mat[c2][c1] = -f64::sin(degrees.to_radians());
            new_mat[c2][c2] =  f64::cos(degrees.to_radians());

            new_mat
        }

        /// Creates a square rotation [Matrix] typically used to rotate [Vector]s
        pub fn rotation_r(c1: usize, c2: usize, radians: f64) -> Self {
            let mut new_mat = Self::identity();

            new_mat[c1][c1] =  f64::cos(radians);
            new_mat[c1][c2] =  f64::sin(radians);
            new_mat[c2][c1] = -f64::sin(radians);
            new_mat[c2][c2] =  f64::cos(radians);

            new_mat
        }
    }

    impl<const X: usize, const Y: usize> Default for Matrix<X, Y> {
        fn default() -> Self {
            Self::new([[0.; Y]; X])
        }
    }

    impl<const X: usize, const Y: usize> std::ops::Index<usize> for Matrix<X, Y> {
        type Output = [f64; Y];
        fn index(&self, index: usize) -> &Self::Output {
            self.values.get(index).unwrap()
        }
    }

    impl<const X: usize, const Y: usize> std::ops::IndexMut<usize> for Matrix<X, Y> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            self.values.get_mut(index).unwrap()
        }
    }

    impl<const X: usize, const Y: usize> std::ops::Neg for Matrix<X, Y> {
        type Output = Self;
        fn neg(self) -> Self::Output {
            let mut new_mat = self;
            for x in 0..X {
                for y in 0..Y {
                    new_mat[x][y] = -new_mat[x][y]
                }
            }
            new_mat
        }
    }

    impl<const X: usize, const Y: usize> std::ops::Add for Matrix<X, Y> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            let mut new_mat = self;
            for x in 0..X {
                for y in 0..Y {
                    new_mat[x][y] += rhs[x][y]
                }
            }
            new_mat
        }
    }

    impl<const X: usize, const Y: usize> std::ops::AddAssign for Matrix<X, Y> {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }

    impl<const X: usize, const Y: usize> std::ops::Sub for Matrix<X, Y> {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            let mut new_mat = self;
            for x in 0..X {
                for y in 0..Y {
                    new_mat[x][y] -= rhs[x][y]
                }
            }
            new_mat
        }
    }

    impl<const X: usize, const Y: usize> std::ops::SubAssign for Matrix<X, Y> {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }

    impl<const X: usize, const Y: usize, const Z: usize> std::ops::Mul<Matrix<X, Z>> for Matrix<Z, Y> {
        type Output = Matrix<X, Y>;
        fn mul(self, rhs: Matrix<X, Z>) -> Self::Output {
            let mut new_mat = Self::Output::default();

            for y in 0..Y {
                for x in 0..X {
                    let mut sum = 0.;
                    for z in 0..Z {
                        sum += self[z][y] * rhs[x][z]
                    }
                    new_mat[x][y] = sum
                }
            }

            new_mat
        }
    }

    impl<const X: usize, const Y: usize> std::ops::Mul<f64> for Matrix<X, Y> {
        type Output = Self;

        fn mul(self, rhs: f64) -> Self::Output {
            let mut new_mat = self;
            for x in 0..X {
                for y in 0..Y {
                    new_mat[x][y] *= rhs
                }
            }
            new_mat
        }
    }

    impl<const X: usize, const Y: usize> std::ops::MulAssign<f64> for Matrix<X, Y> {
        fn mul_assign(&mut self, rhs: f64) {
            *self = *self * rhs
        }
    }

    impl<const X: usize, const Y: usize> std::ops::Mul<Matrix<X, Y>> for f64 {
        type Output = Matrix<X, Y>;

        fn mul(self, rhs: Matrix<X, Y>) -> Self::Output {
            let mut new_mat = rhs;
            for x in 0..X {
                for y in 0..Y {
                    new_mat[x][y] *= self
                }
            }
            new_mat
        }
    }

    impl<const X: usize, const Y: usize> std::ops::Div<f64> for Matrix<X, Y> {
        type Output = Self;
        fn div(self, rhs: f64) -> Self::Output {
            let mut new_mat = self;

            for x in 0..X {
                for y in 0..Y {
                    new_mat[x][y] /= rhs
                }
            }

            new_mat
        }
    }

    impl<const X: usize, const Y: usize> std::ops::DivAssign<f64> for Matrix<X, Y> {
        fn div_assign(&mut self, rhs: f64) {
            *self = *self / rhs
        }
    }

    impl<const X: usize, const Y: usize> std::ops::Div<Matrix<X, Y>> for f64 {
        type Output = Matrix<X, Y>;

        fn div(self, rhs: Matrix<X, Y>) -> Self::Output {
            let mut new_mat = rhs;
            for x in 0..X {
                for y in 0..Y {
                    new_mat[x][y] = self / new_mat[x][y]
                }
            }
            new_mat
        }
    }
}

mod vector {
    use crate::vec::Matrix;

    /// This [Vector] struct is used to represent a mathematical vector used in various mathematical operations
    #[derive(Debug, Copy, Clone)]
    pub struct Vector<const D: usize> {
        pub values: [f64; D]
    }

    impl<const D: usize> Vector<D> {
        /// Creates a new [Vector] struct from a 1d [Array] of [f64]s
        pub fn new(vec: [f64; D]) -> Self {
            Self { values: vec }
        }

        /// Creates a [Matrix] from this [Vector] in a vertical orientation
        pub fn as_v_mat(&self) -> Matrix<1, D> {
            Matrix::new([self.values])
        }

        /// Creates a [Matrix] from this [Vector] in a horizontal orientation
        pub fn as_h_mat(&self) -> Matrix<D, 1> {
            Matrix::new(self.values.map(|x| [x]))
        }

        /// Creates a [Vector] from a [Matrix] in a vertical orientation
        pub fn from_v_mat(mat: Matrix<1, D>) -> Self {
            Vector::new(mat[0])
        }

        /// Creates a [Vector] from a [Matrix] in a horizontal orientation
        pub fn from_h_mat(mat: Matrix<D, 1>) -> Self {
            Vector::new(mat.values.map(|[x]| x))
        }

        /// Returns the length of the D dimensional [Vector]
        pub fn length(&self) -> f64 {
            f64::sqrt(self.values.map(|x| x*x).iter().sum())
        }

        /// Returns the distance between two points represented by two [Vector]s
        pub fn distance(&self, rhs: Self) -> f64 {
            (*self - rhs).length()
        }
    }

    impl Vector<2> {
        pub fn angle_d(&self) -> f64 {
            self[1].atan2(self[0]).to_degrees()
        }

        pub fn angle_r(&self) -> f64 {
            self[1].atan2(self[0])
        }
    }

    impl<const D: usize> Default for Vector<D> {
        fn default() -> Self {
            Self::new([0.; D])
        }
    }

    impl<const D: usize> std::ops::Index<usize> for Vector<D> {
        type Output = f64;
        fn index(&self, index: usize) -> &Self::Output {
            self.values.get(index).unwrap()
        }
    }

    impl<const D: usize> std::ops::IndexMut<usize> for Vector<D> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            self.values.get_mut(index).unwrap()
        }
    }

    impl<const D: usize> std::ops::Neg for Vector<D> {
        type Output = Self;
        fn neg(self) -> Self::Output {
            let mut new_vec = self;

            for i in 0..D {
                new_vec[i] = -new_vec[i]
            }

            new_vec
        }
    }

    impl<const D: usize> std::ops::Add for Vector<D> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            let mut new_vec = self;

            for i in 0..D {
                new_vec[i] += rhs[i]
            }

            new_vec
        }
    }

    impl<const D: usize> std::ops::AddAssign for Vector<D> {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }

    impl<const D: usize> std::ops::Sub for Vector<D> {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            let mut new_vec = self;

            for i in 0..D {
                new_vec[i] -= rhs[i]
            }

            new_vec
        }
    }

    impl<const D: usize> std::ops::SubAssign for Vector<D> {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }

    impl<const D: usize> std::ops::Mul for Vector<D> {
        type Output = f64;

        fn mul(self, rhs: Self) -> Self::Output {
            let mut sum = 0.;

            for i in 0..D {
                sum += self[i] * rhs[i]
            }

            sum
        }
    }

    impl<const D: usize> std::ops::Mul<Matrix<D, D>> for Vector<D> {
        type Output = Self;
        fn mul(self, rhs: Matrix<D, D>) -> Self::Output {
            Vector::from_h_mat(self.as_h_mat() * rhs)
        }
    }

    impl<const D: usize> std::ops::Mul<Vector<D>> for Matrix<D, D> {
        type Output = Vector<D>;
        fn mul(self, rhs: Vector<D>) -> Self::Output {
            Vector::from_v_mat(self * rhs.as_v_mat())
        }
    }

    impl<const D: usize> std::ops::Mul<f64> for Vector<D> {
        type Output = Self;

        fn mul(self, rhs: f64) -> Self::Output {
            let mut new_vec = self;

            for i in 0..D {
                new_vec[i] *= rhs
            }

            new_vec
        }
    }

    impl<const D: usize> std::ops::MulAssign<f64> for Vector<D> {
        fn mul_assign(&mut self, rhs: f64) {
            *self = *self * rhs
        }
    }

    impl<const D: usize> std::ops::Mul<Vector<D>> for f64 {
        type Output = Vector<D>;
        fn mul(self, rhs: Vector<D>) -> Self::Output {
            let mut new_vec = rhs;

            for i in 0..D {
                new_vec[i] *= self
            }

            new_vec
        }
    }

    impl<const D: usize> std::ops::Div<f64> for Vector<D> {
        type Output = Self;
        fn div(self, rhs: f64) -> Self::Output {
            let mut new_vec = self;

            for i in 0..D {
                new_vec[i] /= rhs
            }

            new_vec
        }
    }

    impl<const D: usize> std::ops::DivAssign<f64> for Vector<D> {
        fn div_assign(&mut self, rhs: f64) {
            *self = *self / rhs
        }
    }

    impl<const D: usize> std::ops::Div<Vector<D>> for f64 {
        type Output = Vector<D>;
        fn div(self, rhs: Vector<D>) -> Self::Output {
            let mut new_vec = rhs;

            for i in 0..D {
                new_vec[i] = self / new_vec[i]
            }

            new_vec
        }
    }
}

#[allow(unused_imports)]
pub use {vector::Vector, matrix::Matrix};