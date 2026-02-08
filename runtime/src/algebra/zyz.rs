use approx::AbsDiffEq;
use libm::{cos, sin};
use num::complex::ComplexFloat;
use crate::algebra::{EPSILON, Mat2, phase_angle, su};

pub struct ZyzDecomposition {
    pub lambda: f64,
    pub phi: f64,
    pub theta: f64,
    pub phase_angle: f64,
}

impl ZyzDecomposition {
    pub fn new(mat: &Mat2) -> Self {
        let mut alpha = phase_angle(mat);
        let su = su::<2>(mat);
        // φ = {
        //  2arccos(|U_00|) if |U_00| >= |U_01|
        //  2arcsin(|U_01|) if |U_00| <  |U_01|
        // }
        let phi = if su[(0, 0)].abs() > su[(1, 0)].abs() {
            -2.0 * su[(0, 0)].abs().min(1.0).acos()
        } else {
            -2.0 * su[(1, 0)].abs().min(1.0).asin()
        };

        let (theta, lambda) = {
            let theta_plus_lambda = if (phi / 2.0).cos().abs_diff_eq(&0.0, EPSILON) {
                0.0 // if cos(φ/2) == 0 then θ + λ = 0
            } else {
                // else θ + λ = 2arctan2(Im(U_11 / cos(φ/2), Re(U_11 / cos(φ/2))
                let z = su[(1, 1)] / cos(0.5 * phi);
                2.0 * f64::atan2(z.im, z.re)
            };
            let theta_minus_lambda = if (phi / 2.0).sin().abs_diff_eq(&0.0, EPSILON) {
                0.0 // if sin(φ*1/2) == 0 then θ - λ = 0
            } else {
                // else θ + λ = 2arctan2(Im(U_10 / sin(φ/2), Re(U_10 / sin(φ/2))
                let z = su[(1, 0)] / sin(0.5 * phi);
                2.0 * f64::atan2(z.im, z.re)
            };
            // θ = ((θ + λ) + (θ - λ)) / 2
            let theta = (theta_plus_lambda + theta_minus_lambda) / 2f64;
            // λ = ((θ + λ) - (θ - λ)) / 2
            let lambda = (theta_plus_lambda - theta_minus_lambda) / 2f64;

            (theta, lambda)
        };

        // alpha += (theta + lambda) / 2.0;

        assert!(!lambda.is_nan() && !phi.is_nan() && !theta.is_nan() && !alpha.is_nan());
        ZyzDecomposition { lambda, phi, theta, phase_angle: alpha }
    }
}
