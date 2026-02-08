use crate::qubit::qubit_set::QubitSet;
use crate::qubits;

#[test]
fn test_qubit_set() {
    let mut qubits = QubitSet::from(qubits![0, 1, 2, 4, 5, 6, 9]);
    qubits.add(&qubits![3, 7, 8]);
    assert_eq!(qubits.to_vec(), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    qubits -= qubits![3, 4, 5].into();
    assert_eq!(qubits.to_vec(), vec![0, 1, 2, 6, 7, 8, 9]);
    qubits += qubits![10, 11].into();
    assert_eq!(qubits.to_vec(), vec![0, 1, 2, 6, 7, 8, 9, 10, 11]);
    qubits -= qubits![4, 5, 6, 7, 8].into();
    assert_eq!(qubits.to_vec(), vec![0, 1, 2, 9, 10, 11]);
    assert_eq!(qubits.pop(), Some(0));
    assert_eq!(qubits.pop(), Some(1));
    assert_eq!(qubits.pop(), Some(2));
    assert_eq!(qubits.to_vec(), vec![9, 10, 11]);
}
