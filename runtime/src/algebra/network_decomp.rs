use std::mem::size_of_val;
use nalgebra::{DefaultAllocator, Dim, DimDiff, DimMin, DimSub, U1};
use nalgebra::allocator::Allocator;
use num::complex::{Complex64, ComplexFloat};
use crate::algebra::{DMat, Mat, MatSqrt};
use crate::qubit::QubitAddr;

fn gray_codes(n: u32) -> Vec<u32> {
    (0u32 .. (1u32 << n)).map(|i| i ^ (i >> 1)).collect()
}

/// https://arxiv.org/pdf/quant-ph/9503016v1.pdf
pub struct NetworkDecomposition<D: Dim>
where
    DefaultAllocator:
        Allocator<Complex64, D> +
        Allocator<Complex64, D, D>,
{
    v_mat: Mat<D>,
    vd_mat: Mat<D>,
    circuit: Vec<NetworkDecompositionNode>,
}

pub enum NetworkDecompositionNode {
    CNot { ctrl: QubitAddr, target: QubitAddr },
    CtrlVGate { ctrl: QubitAddr },
    CtrlVDGate { ctrl: QubitAddr },
}

impl<D: Dim + DimSub<U1> + DimMin<D, Output = D>> NetworkDecomposition<D>
where
    DefaultAllocator:
        Allocator<Complex64, D> +
        Allocator<Complex64, D, D> +
        Allocator<Complex64, D, DimDiff<D, U1>> +
        Allocator<Complex64, DimDiff<D, U1>>,
    DefaultAllocator: Allocator<bool, D, D>
{
    fn new(u: &Mat<D>, num_ctrl: usize) -> Self {

        // V = U^\frac{1}{2^{n-1}}, where n is the number of control qubits
        let v_mat = (0 .. num_ctrl - 1).fold(u.clone(), |mat, _| mat.mat_sqrt());
        assert!(v_mat.iter().all(|val| !val.re.is_nan() && !val.im.is_nan()));

        use NetworkDecompositionNode::*;

        let mut circuit: Vec<NetworkDecompositionNode> = vec![CtrlVDGate { ctrl: 0 }];

        // The current state of each control qubits
        // e.g. if current[2] = 0b1100, then the 2th control qubit is 1
        //  only when the 2nd and 3rd qubits are 1. (index starts from 0)
        let mut current: Vec<u32> = (0 .. num_ctrl).map(|i| 1 << i).collect();

        for gray_code in gray_codes(num_ctrl as u32).into_iter().skip(2) {
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
            circuit.push(CNot { ctrl: diff_bit as QubitAddr, target: target_bit as QubitAddr });

            // Change current[target_bit] to gray_code
            current[target_bit] = gray_code;

            // Count one bits in gray_code
            if gray_code.count_ones() % 2 == 1 {
                // Controlled V(target_bit, self.target)
                circuit.push(CtrlVGate { ctrl: target_bit as QubitAddr });
            } else {
                // Controlled VD(target_bit, self.target)
                circuit.push(CtrlVDGate { ctrl: target_bit as QubitAddr });
            }
        }

        Self {
            v_mat: v_mat.clone(),
            vd_mat: v_mat.adjoint(),
            circuit
        }
    }
}
