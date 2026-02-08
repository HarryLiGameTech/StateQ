use std::f64::consts::PI;
use num::complex::Complex64;
use crate::gate::elementary::SingleGate;
use crate::{into_variant, mat2};
use crate::gate::standard::StandardSingleGate;
use crate::algebra::Mat2;
use crate::operation::controlled::ControlledOperation;
use crate::operation::controlled::mux::MuxOperation;
use crate::operation::elementary::ElementaryOperation;
use crate::operation::{Operation, TargetSingle};
use crate::operation::controlled::mux::rotation::MuxRotationOperation;
use crate::operation::controlled::single_ctrl::CtrlSingleTargetOperation;
use crate::qubit::qubit_set::QubitSet;

#[derive(Clone, Debug)]
pub struct SingleTargetMuxOperation {
    pub gates: Vec<SingleGate>,
    pub ctrl: QubitSet,
    pub target: TargetSingle,
}

impl SingleTargetMuxOperation {

    pub fn new(gates: Vec<SingleGate>, ctrl: QubitSet, target: TargetSingle) -> Self {
        Self { gates, ctrl, target }
    }

    fn demutiplex_mat(a: Mat2, b: Mat2) -> (Mat2, Mat2, f64) {
        // The notation is chosen as in https://arxiv.org/pdf/quant-ph/0410066.pdf.
        let x = a * b.adjoint();
        let x_det = x.determinant();
        let x11 = x[(0, 0)] / x_det.sqrt();
        let phi = x_det.to_polar().1; // phase angle of det(x)
        let r1 = ((PI / 2f64 - phi / 2f64 - x11.to_polar().1) * Complex64::i()).exp();
        let r2 = ((PI / 2f64 - phi / 2f64 - x11.to_polar().1 + PI) * Complex64::i()).exp();
        let r = mat2! { r1, 0f64; 0f64, r2; };
        // let eigen = Eigen::new(r * x * r, false, true).unwrap();
        // let d = eigen.eigenvalues;
        // let u = eigen.eigenvectors.unwrap();
        // If d is not equal to diag(i,-i), then we put it into this "standard" form
        // (see eq. (13) in https://arxiv.org/pdf/quant-ph/0410066.pdf) by interchanging
        // the eigenvalues and eigenvectors.
        // let (d, u) = if (d[0] + Complex64::i()).abs() < 1e-10 {
        //     let flip_mat = mat2! { 0f64, 1f64; 0f64, 1f64; };
        //     (d * flip_mat, u * flip_mat)
        // } else {
        //     (d, u)
        // };
        // let d = d.map(|x| x.sqrt()).diagonal();
        // let v = d * u.adjoint() * r.adjoint() * b;
        todo!()
    }

    fn demultiplex(&self) -> (Self, Self, MuxRotationOperation) {
        assert!(self.ctrl.size() > 1);
        let (u1_gates, u2_gates) = self.gates.split_at(self.gates.len() / 2);
        let mut remained_ctrl = self.ctrl.clone();
        let ctrl = remained_ctrl.pop().unwrap();
        let u1 = Self::new(u1_gates.into(), remained_ctrl.clone(), self.target);
        let u2 = Self::new(u2_gates.into(), remained_ctrl, self.target);
        todo!()
    }

    pub fn decompose(&self) -> Vec<ElementaryOperation> {
        use StandardSingleGate::*;
        use SingleGate::*;
        if self.ctrl.size() == 1 && matches!(self.gates[0], Standard(gate) if gate == I) {
            return CtrlSingleTargetOperation::new(
                self.gates[0].clone(),
                self.ctrl.first().unwrap(),
                self.target
            ).decompose();
        }
        todo!()
    }
}

into_variant! {
    SingleTargetMuxOperation 
        => MuxOperation::SingleTarget 
        => ControlledOperation::Mux 
        => Operation::Controlled;
}
