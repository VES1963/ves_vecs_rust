#[derive(Debug, Copy, Clone)]
pub struct Vec2d {
    /// The **x** component of the [Vec2d]
    pub x: f64,
    /// The **y** component of the [Vec2d]
    pub y: f64,
}

impl Vec2d {
    /// Creates a new [Vec2d] from an **x** and a **y** component
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    /// Returns the length of the [Vec2d]
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    /// Returns a normalized version of itself
    pub fn normalise(&self) -> Self {
        *self/self.length()
    }
}

impl Into<(f64, f64)> for Vec2d {
    fn into(self) -> (f64, f64) {
        (self.x, self.y)
    }
}

impl Into<[f64;2]> for Vec2d {
    fn into(self) -> [f64; 2] {
        [self.x, self.y]
    }
}

impl std::ops::Add for Vec2d {
    type Output = Self;
    /// Adds two [Vec2d]:s together and returns the resulting [Vec2d]
    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::ops::AddAssign for Vec2d {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub for Vec2d {
    type Output = Self;
    /// Subtracts the right hand side [Vec2d] from the left hand side [Vec2d] and returns the resulting [Vec2d]
    fn sub(self, rhs: Self) -> Self {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl std::ops::SubAssign for Vec2d {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::Mul for Vec2d {
    type Output = f64;
    /// Multiplies two [Vec2d]:s together and returns their dot product
    fn mul(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl std::ops::Mul<f64> for Vec2d {
    type Output = Self;
    /// Multiplies the [Vec2d] by a scalar value and returns the resulting [Vec2d]
    fn mul(self, rhs: f64) -> Self {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

impl std::ops::MulAssign<f64> for Vec2d {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl std::ops::Mul<Vec2d> for f64 {
    type Output = Vec2d;
    /// Multiplies the [Vec2d] by the scalar value and returns the resulting [Vec2d]
    fn mul(self, rhs: Vec2d) -> Vec2d { rhs * self }
}

impl std::ops::Div<f64> for Vec2d {
    type Output = Self;
    /// Divides the [Vec2d] by the scalar value and returns the resulting [Vec2d]
    fn div(self, rhs: f64) -> Self {
        Self { x: self.x / rhs, y: self.y / rhs }
    }
}

impl std::ops::DivAssign<f64> for Vec2d {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl std::ops::Div<Vec2d> for f64 {
    type Output = Vec2d;
    /// Divides the scalar value by the [Vec2d] and returns the resulting [Vec2d]
    fn div(self, rhs: Vec2d) -> Vec2d {
        Vec2d::new(self/rhs.x, self/rhs.y)
    }
}

impl PartialEq for Vec2d {
    /// Checks if self is equal to other
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v1 = Vec2d::new(3., 4.);
        assert_eq!(v1.length(), 5.);
        println!("v1: {v1:?}");
        let v2 = Vec2d::new(4., 3.);
        assert_eq!(v2.length(), v1.length());
        println!("v2: {v2:?}");
        let v3 = v1 + v2;
        assert_eq!(v3, Vec2d::new(7., 7.));
        println!("{v1:?} + {v2:?} = {v3:?}");
        let v4 = v3 - v2;
        assert_eq!(v4, v1);
        println!("{v3:?} - {v2:?} = {v4:?}");
        let d = v4 * v2;
        assert_eq!(d, 24.);
        println!("{v4:?} * {v2:?} = {d:?}");
        let n = v4.normalise();
        assert_eq!(n, Vec2d::new(0.6, 0.8));
        assert_eq!(n.length(), 1.);
        println!("{v4:?} / {:?} = {n:?}", v4.length());
    }
}
