use num::complex::Complex64;
use num_traits::abs;
use crate::algebra::Mat2;
use crate::decompose::single::{EulerDecomposition, zyz_decompose};

macro_rules! zyz_test {
    {
        [
            $u11re: literal + $u11im: literal j,
            $u12re: literal + $u12im: literal j,
            $u21re: literal + $u21im: literal j,
            $u22re: literal + $u22im: literal j,
        ] => ($theta: literal, $phi: literal, $lambda: literal)
    } => {
        let u11 = Complex64::new($u11re, $u11im);
        let u12 = Complex64::new($u12re, $u12im);
        let u21 = Complex64::new($u21re, $u21im);
        let u22 = Complex64::new($u22re, $u22im);
        let mat = Mat2::new(u11, u12, u21, u22);
        use crate::gate::rotation::Rotation::*;
        if let EulerDecomposition(Rz(theta0), Ry(phi0), Rz(lambda0), _) = zyz_decompose(&mat) {
            println!("theta: {}, phi: {}, lambda: {}", theta0, phi0, lambda0);
            assert!(abs($theta - theta0) < 1e-6);
            assert!(abs($phi - phi0) < 1e-6);
            assert!(abs($lambda - lambda0) < 1e-6);
        } else {
            assert!(false);
        }
    };
}

#[test]
fn test_zyz_1() {
    zyz_test! {
        [
            -0.48462354 + -0.21905938 j,
            0.75355429 + -0.38640516 j,
            0.47356705 + -0.7020593 j,
            -0.15966309 + -0.50730137 j,
        ] => (
            -4.039941613013334, -2.0200640353825903, -1.401895001061111
        )
    }
}

#[test]
fn test_zyz_2() {
    zyz_test! {
        [
            -0.64579398 + -0.73873446 j,
            0.18907107 + 0.03838828 j,
            0.06554469 + 0.18145364 j,
            0.82499737 + 0.53118529 j,
        ] => (
            2.489480450885276, -0.38829235571840715, 0.37173558649579985
        )
    }
}

