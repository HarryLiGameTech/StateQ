use crate::experimental::operation::elementary::standard::StandardGate;
use crate::experimental::operation::Operation;

pub struct Circuit<T: Operation> {
    operations: Vec<T>,
}

// impl<T: Sized + Operation> Circuit<T> {
//     fn new(operations: Vec<T>) -> Self {
//         Self {
//             operations: operations.into_iter().map(|op| Box::new(op)).collect()
//         }
//     }
// }
//
// impl<T: ?Sized + Operation> Circuit<T> {
//     pub fn new_dyn(operations: Vec<Box<T>>) -> Self {
//         Self { operations }
//     }
//
//     pub fn into<R: Operation + From<T>>(self) -> Circuit<R> where T: Sized {
//         Circuit::<R>::new_dyn(self.operations.into_iter().map(|op| {
//             Box::new(R::from(*op))
//         }).collect())
//     }
//
//     pub fn dyn_into<R: Operation + From<Box<T>>>(self) -> Circuit<R> {
//         Circuit::<R>::new_dyn(self.operations.into_iter().map(|op| {
//             Box::new(R::from(op))
//         }).collect())
//     }
//
//     pub fn add(&mut self, operation: Box<T>) {
//         self.operations.push(operation);
//     }
// }

impl <OP: Operation> Circuit<OP> {

    pub fn new() -> Self {
        Self { operations: vec![] }
    }

    pub fn add(&mut self, operation: OP) {
        self.operations.push(operation);
    }
}
