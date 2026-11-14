mod vec;

#[cfg(test)]
mod tests {
    use super::vec::*;

    #[test]
    fn it_works() {
        let v1 = Vector::new([3., 4.]);
        let m1 = Matrix::new([[0., 1.], [-1., 0.]]);

        println!("v1: {v1:?}, v: {}", v1.angle_d());
        println!("*");
        println!("m1:\n{m1:?}");

        let v2 = v1 * m1.transpose();

        println!("=");
        println!("v2: {v2:?}, v: {}", v2.angle_d());
    }
}
