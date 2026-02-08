use std::mem::size_of_val;
use crate::gate::{Dagger, DoubleTargetGate};
use crate::gate::elementary::{ElementaryGate, is_identity, SingleGate};
use crate::gate::standard::StandardDoubleGate::CX;
use crate::gate::unitary::UnitarySingleGate;
use crate::algebra::{mat_sqrt, ToMat2};
use crate::operation::controlled::single_ctrl::CtrlSingleTargetOperation;
use crate::operation::elementary::ElementaryOperation;
use crate::qubit::QubitAddr;
use crate::qubit::qubit_accessor::QubitAccessor;
use crate::qubit::qubit_set::QubitSet;

#[derive(Clone, Debug)]
pub struct MultiCtrlSingleTargetOperation {
    gate: SingleGate,
    ctrl: QubitSet,
    target: QubitAddr,
}

impl MultiCtrlSingleTargetOperation {
    pub fn new(gate: SingleGate, ctrl: QubitSet, target: QubitAddr) -> Self {
        Self { gate, ctrl, target }
    }
}

impl Dagger for MultiCtrlSingleTargetOperation {
    fn dagger(self) -> Self {
        Self {
            gate: self.gate.dagger(),
            ctrl: self.ctrl,
            target: self.target,
        }
    }
}

fn gray_codes(n: u32) -> Vec<u32> {
    (0u32 .. (1u32 << n)).map(|i| i ^ (i >> 1)).collect()
}

impl MultiCtrlSingleTargetOperation {

    pub fn decompose(&self) -> Vec<ElementaryOperation> {
        if is_identity(&self.gate) {
            vec![/* no need for decomposing I */]
        } else if self.ctrl.size() == 1 {
            CtrlSingleTargetOperation::new(
                self.gate.clone(), self.ctrl.first().unwrap(), self.target
            ).decompose()
        } else {
            self.network_decompose()
        }
    }

    /// https://arxiv.org/pdf/quant-ph/9503016v1.pdf
    fn network_decompose(&self) -> Vec<ElementaryOperation> {

        // println!("Network decompose: {:?}", self);

        let ctrls = self.ctrl.to_vec();

        let u = self.gate.to_mat2();

        // V = U^\frac{1}{2^{n-1}}, where n is the number of control qubits
        let v_mat = (0 .. ctrls.len() - 1).fold(u, |mat, _| mat_sqrt(&mat));

        assert!(v_mat.iter().all(|val| !val.re.is_nan() && !val.im.is_nan()));

        let v = UnitarySingleGate(Box::new(v_mat));
        let vd = UnitarySingleGate(Box::new(v_mat.adjoint()));
        // println!("{}", v_mat);
        // println!("{}", vd.to_mat2());

        let mut result: Vec<ElementaryOperation> = CtrlSingleTargetOperation::new(
            v.clone(), ctrls[0], self.target
        ).decompose();
        // println!("V({}, {})", ctrls[0], self.target);

        // The current state of each control qubits
        // e.g. if current[2] = 0b1100, then the 2th control qubit is 1
        //  only when the 2nd and 3rd qubits are 1. (index starts from 0)
        let mut current: Vec<u32> = (0 .. ctrls.len()).map(|i| 1 << i).collect();

        for gray_code in gray_codes(ctrls.len() as u32).into_iter().skip(2) {
            // The highest 1 of gray_code
            let target_bit = size_of_val(&gray_code) * 8 - gray_code.leading_zeros() as usize - 1;
            // We want current[target_bit] to be gray_code (quantum state),
            //  so we need to find the different bit between gray_code and current[target_bit]
            let diff_bit = (gray_code ^ current[target_bit]).trailing_zeros() as usize;
            // If we want to change current[target_bit] to gray_code (quantum state),
            //  we need to apply CNot on (diff_bit, target_bit).
            // For example, if current target bit is 1, current[target_bit] = 0b0010
            //  and we want to change it to 0b0011, then we need to apply CNot(q[0], q[1]),
            //  because the different bit is 0 and the target bit (highest bit 1) is 1.
            // This CNot can help us to "merge" the state of q[0] and q[1],
            //  so that we can apply V (or V dagger) on the "merged" state (q[0] && q[1]).
            result.push(CX.apply_to(
                (ctrls[diff_bit] as QubitAddr, ctrls[target_bit] as QubitAddr)
            ).into());
            // println!("CX({}, {})", ctrls[diff_bit] as QubitAddr, ctrls[target_bit] as QubitAddr);

            // Change current[target_bit] to gray_code
            current[target_bit] = gray_code;

            // Count one bits in gray_code
            if gray_code.count_ones() % 2 == 1 {
                // Controlled V(target_bit, self.target)
                // println!("V({}, {})", ctrls[target_bit], self.target);
                let mut dec_ctrl_v = CtrlSingleTargetOperation::new(
                    v.clone(), ctrls[target_bit], self.target
                ).decompose();
                // println!("Decompose Ctrl V: {:?}", dec_ctrl_v);
                result.append(&mut dec_ctrl_v);
            } else {
                // Controlled VD(target_bit, self.target)
                // println!("VD({}, {})", ctrls[target_bit], self.target);
                let mut dec_ctrl_vd = CtrlSingleTargetOperation::new(
                    vd.clone(), ctrls[target_bit], self.target
                ).decompose();
                // println!("Decompose Ctrl VD: {:?}", dec_ctrl_vd);
                result.append(&mut dec_ctrl_vd);
            }
        }

        // println!("{:?}", result);
        result
    }
}

/// Multi-controlled multi-target
#[derive(Clone)]
pub struct MultiCtrlMultiTargetOperation {
    pub gate: ElementaryGate,
    pub ctrl: QubitAccessor,
    pub target: QubitAccessor,
}

impl MultiCtrlMultiTargetOperation {
    pub fn new(gate: ElementaryGate, ctrl: QubitAccessor, target: QubitAccessor) -> Self {
        Self { gate, ctrl, target }
    }
}

impl Dagger for MultiCtrlMultiTargetOperation {
    fn dagger(self) -> Self {
        Self {
            gate: self.gate.dagger(),
            ctrl: self.ctrl,
            target: self.target,
        }
    }
}
