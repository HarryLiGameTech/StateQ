use crate::algebra::Mat2;
use crate::algebra::zyz::ZyzDecomposition;

pub struct AbcDecomposition {
    a_rz_ry: (f64, f64),
    b_ry_rz: (f64, f64),
    c_rz: f64,
}

impl AbcDecomposition {
    pub fn new(mat: &Mat2) -> Self {
        let ZyzDecomposition {
            theta, phi, lambda, phase_angle: _phase_angle
        } = ZyzDecomposition::new(mat);
        Self {
            a_rz_ry: (lambda, 0.5 * phi),
            b_ry_rz: (-0.5 * phi, -0.5 * (theta + lambda)),
            c_rz: 0.5 * (theta - lambda),
        }
    }
}
