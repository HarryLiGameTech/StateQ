use nalgebra::{Const, Matrix4, SVD};
use num::complex::Complex64;
use crate::{c64, const_mat4, mat4};
use crate::gate::custom::CustomSingleGate;
use crate::algebra::{Mat4, su};
use crate::operation::{DoubleTargetOperation, Operation, SingleTargetOperation};
use crate::operation::elementary::custom::{CustomOperation, CustomSingleOperation};
use crate::operation::elementary::ElementaryOperation;
use crate::operation::elementary::unitary::UnitarySingleOperation;

const B_NONNORM_MAT: Mat4 = const_mat4! {
    c64!(1), c64!(1 i), c64!(0), c64!(0);
    c64!(0), c64!(0), c64!(1 i), c64!(1);
    c64!(0), c64!(0), c64!(1 i), c64!(-1);
    c64!(1), c64!(-1 i), c64!(0), c64!(0);
};

const B_NONNORM_MAT_DAGGER: Mat4 = const_mat4! {
    c64!(1), c64!(-1 i), c64!(0), c64!(0);
    c64!(0), c64!(0), c64!(1 i), c64!(1);
    c64!(0), c64!(0), c64!(1 i), c64!(-1);
    c64!(1), c64!(1 i), c64!(0), c64!(0);
};

fn transform_magic_basis(mat: Mat4, reverse: bool) -> Mat4 {
    if reverse {
        B_NONNORM_MAT_DAGGER * mat * B_NONNORM_MAT
    } else {
        B_NONNORM_MAT * mat * B_NONNORM_MAT_DAGGER
    }
}

pub fn kak_decompose(operation: impl DoubleTargetOperation) -> Vec<ElementaryOperation> {
    // TODO: consider global phase
    // reference: https://github.com/qclib/qclib/blob/7ddfb77c67c7b8f4d1f3d612b4e93a9206754cd3/qclib/decompose2q.py#L146
    let u = su::<4>(&operation.to_mat4());
    todo!()
}

/// Decompose a 2-qubit unitary composed of two 1-qubit gates.
/// Uses the "Nearest Kronecker Product" algorithm.
pub fn kronecker_decompose(
    operation: impl DoubleTargetOperation,
    single_decomposer: impl Fn(&UnitarySingleOperation) -> Vec<ElementaryOperation>,
) -> Vec<ElementaryOperation> {
    let u = su::<4>(&operation.to_mat4());
    // from:
    //  | 0,  1,  2,  3 |
    //  | 4,  5,  6,  7 |
    //  | 8,  9, 10, 11 |
    //  |12, 13, 14, 15 |
    // to:
    //  | 0,  1,  4,  5 |
    //  | 2,  3,  6,  7 |
    //  | 8,  9, 12, 13 |
    //  |10, 11, 14, 15 |
    const MAPPING_MAT: Matrix4<usize> = Matrix4::<usize>::new(
        0,  1,  4,  5,
        2,  3,  6,  7,
        8,  9, 12, 13,
        10, 11, 14, 15,
    );
    let mut r = Mat4::from_fn(|r, c| {
        u.as_slice()[MAPPING_MAT[(r, c)]]
    });
    let r_svd = r.svd(true, true);
    let u = r_svd.u.unwrap();
    let singular = r_svd.singular_values;
    let v_t = r_svd.v_t.unwrap();
    // gate0, gate1: Mat2
    let gate0 = u.row(0).scale(singular[0]).reshape_generic(Const::<2>, Const::<2>);
    let gate1 = r.column(0).scale(singular[0]).reshape_generic(Const::<2>, Const::<2>);

    let targets = operation.get_target();

    let decomposed_gate0 = single_decomposer(
        &UnitarySingleOperation::from_mat(gate0, targets.0)
    );
    let decomposed_gate1 = single_decomposer(
        &UnitarySingleOperation::from_mat(gate1, targets.1)
    );

    decomposed_gate0.into_iter().chain(decomposed_gate1.into_iter()).collect()
}
