use crate::algebra::{Mat2, mat_sqrt};

#[test]
fn test_mat_sqrt() {
    let result = mat_sqrt(&Mat2::new(1.0.into(), 2.0.into(), 3.0.into(), 4.0.into()));
    let result = mat_sqrt(&result);
    println!("{}", result);
}

#[test]
fn test_csd() {
    
}
