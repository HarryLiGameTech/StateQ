use std::f64::consts::PI;
use nalgebra::max;
use num::integer::gcd;
use num_traits::Pow;
use rand::Rng;
use crate::bytecode::instruction::Instruction;
use crate::gate::standard::StandardSingleGate::{H, P, X, Z};
use crate::program::builder::QuantumProgramContextBuilder;
use crate::program::QuantumProgramContext;
use crate::qubit::{QubitAddr, Slice};
use crate::qubits;
use crate::backend::{execute_bytecode, ExecuteResult};
use crate::bytecode::ByteCode;
use crate::gate::standard::StandardDoubleGate::{CP, CX, SWP};
use crate::measurement::MeasurementResultEntry;
use crate::qubit::ctrl_qubit_set::ControlQubitSet;
use crate::qubit::qubit_accessor::QubitAccessor;

fn get_ctx_with_default_passes() -> QuantumProgramContext {
    let mut ctx_builder = QuantumProgramContextBuilder::new();
    ctx_builder.default_passes();
    ctx_builder.build()
}

fn print_instructions(instructions: &[Instruction]) {
    for instruction in instructions {
        println!("{:?}", instruction)
    }
}

#[test]
fn test_simple_circuit() {
    let mut ctx = get_ctx_with_default_passes();
    ctx.enter();
    let alloc = ctx.alloc(1);
    let target = alloc.borrow()[0];
    ctx.push(H, qubits![target]);
    ctx.measure(qubits![target]);
    ctx.exit();
    let instructions = ctx.compile_circuit();
    print_instructions(&instructions);
    let result = execute_bytecode(instructions.into(), 64);
}

#[test]
fn test_bernstein_vazirani() {
    let mut ctx = get_ctx_with_default_passes();
    const BIT_STR_SIZE: usize = 6;
    const BIT_STR: u64 = 0b101011;
    ctx.enter();
    let alloc = ctx.alloc(BIT_STR_SIZE + 1);
    let qreg = alloc.borrow();
    let output = qreg[BIT_STR_SIZE];
    for i in 0 ..= BIT_STR_SIZE {
        ctx.push(H, qubits![qreg[i]]);
    }
    ctx.push(Z, qubits![output]);
    for i in 0 .. BIT_STR_SIZE {
        if BIT_STR & (1 << i) != 0 {
            ctx.ctrl_qubits.control(&qubits![qreg[i]], true);
            ctx.push(X, qubits![output]);
            ctx.ctrl_qubits.decontrol(&qubits![qreg[i]]);
        }
    }
    for i in 0 .. BIT_STR_SIZE {
        ctx.push(H, qubits![qreg[i]]);
    }
    ctx.measure(qreg.slice(0, (BIT_STR_SIZE - 1) as QubitAddr, 1));
    ctx.exit();
    let instructions = ctx.compile_circuit();
    print_instructions(&instructions);

    const SHOTS: usize = 64;
    let result: ExecuteResult = execute_bytecode(instructions.into(), SHOTS).into();
    assert_eq!(result.measurement.measurements[0].value, BIT_STR);
    assert_eq!(result.measurement.measurements[0].count, SHOTS as u64);
}

#[test]
fn test_deutsch_jozsa_constant() {
    let mut ctx = get_ctx_with_default_passes();
    const BIT_STR_SIZE: usize = 6;
    ctx.enter();
    let alloc = ctx.alloc(BIT_STR_SIZE + 1);
    let qreg = alloc.borrow();
    let output = qreg[BIT_STR_SIZE];

    for i in 0 .. BIT_STR_SIZE {
        ctx.push(H, qubits![qreg[i]]);
    }
    ctx.push(X, qubits![output]);
    ctx.push(H, qubits![output]);

    // Oracle
    let rand_const = rand::thread_rng().gen_range(0u64 ..= 1u64);
    if rand_const == 1 {
        ctx.push(X, qubits![output]);
    }

    for i in 0 .. BIT_STR_SIZE {
        ctx.push(H, qubits![qreg[i]]);
    }
    ctx.measure(qreg.slice(0, (BIT_STR_SIZE - 1) as QubitAddr, 1));

    ctx.exit();

    let instructions = ctx.compile_circuit();
    print_instructions(&instructions);

    const SHOTS: usize = 64;
    let result: ExecuteResult = execute_bytecode(instructions.into(), SHOTS).into();
    assert_eq!(result.measurement.measurements[0].value, 0);
    assert_eq!(result.measurement.measurements[0].count, SHOTS as u64);
}

#[test]
fn test_deutsch_jozsa_balance() {
    let mut ctx = get_ctx_with_default_passes();
    const BIT_STR_SIZE: usize = 6;
    ctx.enter();
    let alloc = ctx.alloc(BIT_STR_SIZE + 1);
    let qreg = alloc.borrow();
    let output = qreg[BIT_STR_SIZE];

    for i in 0..BIT_STR_SIZE {
        ctx.push(H, qubits![qreg[i]]);
    }
    ctx.push(X, qubits![output]);
    ctx.push(H, qubits![output]);

    // Oracle
    let rand_const = rand::thread_rng().gen_range(0u64 ..= 2u64.pow(BIT_STR_SIZE as u32));
    for i in 0 .. BIT_STR_SIZE {
        if rand_const & (1 << i) != 0 {
            ctx.push(X, qubits![output]);
        }
    }
    for i in 0 .. BIT_STR_SIZE {
        ctx.control(qubits![qreg[i]], true);
        ctx.push(X, qubits![output]);
        ctx.decontrol(qubits![qreg[i]]);
    }
    for i in 0 .. BIT_STR_SIZE {
        if rand_const & (1 << i) != 0 {
            ctx.push(X, qubits![output]);
        }
    }

    for i in 0 .. BIT_STR_SIZE {
        ctx.push(H, qubits![qreg[i]]);
    }
    ctx.measure(qreg.slice(0, (BIT_STR_SIZE - 1) as QubitAddr, 1));

    ctx.exit();

    let instructions = ctx.compile_circuit();
    print_instructions(&instructions);

    const SHOTS: usize = 64;
    let result: ExecuteResult = execute_bytecode(instructions.into(), SHOTS).into();
    assert_eq!(result.measurement.measurements[0].value, 0b111111);
    assert_eq!(result.measurement.measurements[0].count, SHOTS as u64);
}

#[test]
fn test_simon() {
    let mut ctx = get_ctx_with_default_passes();
    const BIT_STR_SIZE: usize = 6;
    const BIT_STR: u64 = 0b101100;
    ctx.enter();
    let alloc = ctx.alloc(BIT_STR_SIZE * 2);
    let qreg = alloc.borrow();
    for i in 0 .. BIT_STR_SIZE {
        ctx.push(H, qubits![qreg[i]]);
    }

    // Oracle
    for i in 0 .. BIT_STR_SIZE {
        ctx.control(qubits![qreg[i]], true);
        ctx.push(X, qubits![qreg[i + BIT_STR_SIZE]]);
        ctx.decontrol(qubits![qreg[i]]);
    }
    let lowbit = (BIT_STR as i64) & -(BIT_STR as i64);
    for i in 0 .. BIT_STR_SIZE {
        if BIT_STR & (1 << i) != 0 {
            ctx.control(qubits![lowbit - 1], true);
            ctx.push(X, qubits![qreg[i + BIT_STR_SIZE]]);
            ctx.decontrol(qubits![lowbit - 1]);
        }
    }

    for i in 0 .. BIT_STR_SIZE {
        ctx.push(H, qubits![qreg[i]]);
    }
    ctx.measure(qreg.slice(0, (BIT_STR_SIZE - 1) as QubitAddr, 1));

    ctx.exit();

    let instructions = ctx.compile_circuit();
    print_instructions(&instructions);
    let result: ExecuteResult = execute_bytecode(instructions.into(), 256).into();

    for MeasurementResultEntry { value, count } in result.measurement.measurements {
        assert_eq!((BIT_STR & value).count_ones() % 2, 0);
    }
}

fn qft(ctx: &mut QuantumProgramContext, qreg: &QubitAccessor) {
    ctx.enter();
    let n = qreg.size();
    for i in 0 .. n {
        ctx.push(H, qubits![qreg[n - 1 - i]]);
        for j in i + 1 .. n {
            ctx.push(CP { angle: PI / (1 << (j - i)) as f64 }, qubits![qreg[n-1-j], qreg[n-1-i]])
        }
    }
    for i in 0 .. n / 2 {
        ctx.push(SWP, qubits![qreg[i], qreg[n - i - 1]]);
    }
    ctx.exit();
}

fn inv_qft(ctx: &mut QuantumProgramContext, qreg: &QubitAccessor) {
    ctx.enter();
    let n = qreg.size();
    for i in 0 .. n / 2 {
        ctx.push(SWP, qubits![qreg[i], qreg[n - i - 1]]);
    }
    for i in (0 .. n).rev() {
        for j in (i + 1 .. n).rev() {
            ctx.push(CP { angle: -PI / (1 << (j - i)) as f64 }, qubits![qreg[n-1-j], qreg[n-1-i]])
        }
        ctx.push(H, qubits![qreg[n - 1 - i]]);
    }
    ctx.exit();
}

/// |φ(x)> -> |φ(x+a)>
fn phase_adder(ctx: &mut QuantumProgramContext, qreg: &QubitAccessor, a: i32) {
    ctx.enter();
    let n = qreg.size() as u32;
    let p = 2u32.pow(n) as i32;
    let a = (a % p + p) % p;

    for i in 0 .. n {
        if a >> (n - 1 - i) & 1 != 0 {
            for j in 0 ..= i {
                ctx.push(
                    P { angle: PI * 2f64.pow(j as i32 - i as i32) },
                    qubits![qreg[j as usize]],
                );
            }
        }
    }
    ctx.exit();
}

fn adder(ctx: &mut QuantumProgramContext, qreg: &QubitAccessor, value: i32) {
    ctx.enter();
    let mut empty_ctrl = ControlQubitSet::new();
    // swap(&mut empty_ctrl, &mut ctx.ctrl_qubits);
    ctx.pause_ctrl();
    qft(ctx, qreg);
    // swap(&mut empty_ctrl, &mut ctx.ctrl_qubits);
    ctx.restore_ctrl();
    phase_adder(ctx, qreg, value);
    // swap(&mut empty_ctrl, &mut ctx.ctrl_qubits);
    ctx.pause_ctrl();
    ctx.begin_dagger();
    qft(ctx, qreg);
    ctx.end_dagger();
    // swap(&mut empty_ctrl, &mut ctx.ctrl_qubits);
    ctx.restore_ctrl();
    ctx.exit();
}

fn test_adder_helper(a: i32, b: i32, n: usize) -> i32 {
    let mut ctx = get_ctx_with_default_passes();
    ctx.enter();
    let alloc = ctx.alloc(n);
    let qreg = alloc.borrow();
    for i in 0 .. n {
        if a & (1 << i) != 0 {
            ctx.push(X, qubits![qreg[i]]);
        }
    }
    adder(&mut ctx, &qreg, b);
    ctx.measure(qreg.slice(0, n as QubitAddr, 1));
    ctx.exit();

    let instructions = ctx.compile_circuit();
    print_instructions(&instructions);
    let result: ExecuteResult = execute_bytecode(instructions.into(), 10).into();

    result.measurement.measurements[0].value as i32
}

#[test]
fn test_adder() {
    assert_eq!(test_adder_helper(15, 23, 8), 15 + 23);
    assert_eq!(test_adder_helper(192, 15, 8), 192 + 15);
    assert_eq!(test_adder_helper(233, -156, 8), 233 - 156);
}

fn mod_adder(ctx: &mut QuantumProgramContext, qreg: &QubitAccessor, value: i32, p: i32) {
    ctx.enter();

    let a = (value % p + p) % p;
    let n = qreg.size();
    let overflow_ancilla = ctx.alloc(1);
    let mod_ancilla = ctx.alloc(1);
    let b = qreg.clone() + overflow_ancilla.borrow().clone();

    adder(ctx, &b, a);

    adder(ctx, &b, -p);
    ctx.push(CX, overflow_ancilla.borrow().clone() + mod_ancilla.borrow().clone());

    ctx.control(mod_ancilla.borrow().clone(), true);
    adder(ctx, &b, p);
    ctx.decontrol(mod_ancilla.borrow().clone());

    let mut empty_ctrl = ControlQubitSet::new();
    // swap(&mut ctx.ctrl_qubits, &mut empty_ctrl);
    ctx.pause_ctrl();
    adder(ctx, &b, -a);
    // swap(&mut ctx.ctrl_qubits, &mut empty_ctrl);
    ctx.restore_ctrl();
    ctx.push(X, overflow_ancilla.borrow().clone());
    ctx.push(CX, overflow_ancilla.borrow().clone() + mod_ancilla.borrow().clone());
    ctx.push(X, overflow_ancilla.borrow().clone());
    // swap(&mut ctx.ctrl_qubits, &mut empty_ctrl);
    ctx.pause_ctrl();
    ctx.begin_dagger();
    adder(ctx, &b, -a);
    ctx.end_dagger();
    // swap(&mut ctx.ctrl_qubits, &mut empty_ctrl);
    ctx.restore_ctrl();

    ctx.exit();
}

fn test_mod_adder_helper(a: i32, b: i32, p: i32, n: usize) -> i32 {
    let mut ctx = get_ctx_with_default_passes();
    ctx.enter();
    let alloc = ctx.alloc(n);
    let qreg = alloc.borrow();
    for i in 0 .. n {
        if a & (1 << i) != 0 {
            ctx.push(X, qubits![qreg[i]]);
        }
    }
    mod_adder(&mut ctx, &qreg, b, p);
    ctx.measure(qreg.slice(0, n as QubitAddr, 1));
    ctx.exit();

    let instructions = ctx.compile_circuit();

    print_instructions(&instructions);
    let result: ExecuteResult = execute_bytecode(instructions.into(), 10).into();

    result.measurement.measurements[0].value as i32
}

#[test]
fn test_mod_adder() {
    assert_eq!(test_mod_adder_helper(19, 57, 61, 6), (19 + 57) % 61);
    assert_eq!(test_mod_adder_helper(33, 29, 24, 6), (33 + 29) % 24);
    assert_eq!(test_mod_adder_helper(112, 147, 174, 8), (112 + 147) % 174);
    assert_eq!(test_mod_adder_helper(66, 79, 101, 8), (66 + 79) % 101);
}

fn mod_mult0(
    mut ctx: &mut QuantumProgramContext,
    qreg0: &QubitAccessor, qreg1: &QubitAccessor,
    value: i32, p: i32
) {
    let a = (value % p + p) % p;
    assert_eq!(qreg0.size(), qreg1.size());
    let n = qreg0.size();
    for i in 0 .. n as u32 {
        ctx.control(qubits![qreg0[i as usize]], true);
        mod_adder(ctx, qreg1, ((2i32.pow(i) % p) * a) % p, p);
        ctx.decontrol(qubits![qreg0[i as usize]]);
    }
}

fn exgcd(a: i32, b: i32) -> (i32, i32, i32) {
    if b == 0 {
        (1, 0, a)
    } else {
        let (y, x, g) = exgcd(b, a % b);
        (x, y - x * (a / b), g)
    }
}

fn mod_inv(a: i32, m: i32) -> i32 {
    let (x, y, g) = exgcd(a, m);
    (x % m + m) % m
}

fn mod_multiplier(
    mut ctx: &mut QuantumProgramContext, qreg: &QubitAccessor, value: i32, p: i32
) {
    ctx.enter();
    let a = (value % p + p) % p;
    let b = p - mod_inv(a, p);
    let n = qreg.size();
    let ancilla = ctx.alloc(n);
    mod_mult0(ctx, qreg, &ancilla.borrow().clone(), a, p);
    for i in 0 .. n {
        ctx.push(SWP, qubits![qreg[i], ancilla.borrow()[i]]);
    }
    mod_mult0(ctx, qreg, &ancilla.borrow().clone(), b, p);
    ctx.exit();
}

fn pow_mod(mut base: i32, mut exponent: i32, modular: i32) -> i32 {
    assert!(exponent > 0);
    let mut result = 1;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = (result * base) % modular;
        }
        base = (base * base) % modular;
        exponent >>= 1;
    }
    result
}

fn test_mod_multiplier_helper(a: i32, b: i32, p: i32, n: usize) -> i32 {
    let mut ctx = get_ctx_with_default_passes();
    ctx.enter();
    let alloc0 = ctx.alloc(n);
    let qreg = alloc0.borrow();
    for i in 0 .. n {
        if a & (1 << i) != 0 {
            ctx.push(X, qubits![qreg[i]]);
        }
    }
    mod_multiplier(&mut ctx, &qreg, b, p);
    ctx.measure(qreg.slice(0, n as QubitAddr, 1));
    ctx.exit();

    let instructions = ctx.compile_circuit();

    let shots = 5;
    let result: ExecuteResult = execute_bytecode(instructions.into(), shots).into();

    assert_eq!(result.measurement.measurements[0].count, shots as u64);
    result.measurement.measurements[0].value as i32
}

#[test]
fn test_mod_multiplier() {
    assert_eq!(test_mod_multiplier_helper(7, 9, 11, 4), (7 * 9) % 11);
    assert_eq!(test_mod_multiplier_helper(7, 9, 7, 4), (7 * 9) % 7);
    assert_eq!(test_mod_multiplier_helper(37, 49, 7, 4), (37 * 49) % 7);
    assert_eq!(test_mod_multiplier_helper(19, 57, 61, 6), (19 * 57) % 61);
    assert_eq!(test_mod_multiplier_helper(33, 29, 24, 6), (33 * 29) % 24);
    assert_eq!(test_mod_multiplier_helper(137, 51, 5, 8), (137 * 51) % 5);
    assert_eq!(test_mod_multiplier_helper(137, 51, 115, 8), (137 * 51) % 115);
}

/// shor's algorithm
/// https://arxiv.org/pdf/quant-ph/9508027.pdf
fn shor_period_finder_circuit(ctx: &mut QuantumProgramContext, a: i32, p: i32) {
    let n = p.ilog2() + 1;
    println!("n = {}", n);
    ctx.enter();
    let work = ctx.alloc(2 * n as usize);
    let mult = ctx.alloc(n as usize);

    ctx.push(X, qubits![mult.borrow()[0]]);

    for i in 0 .. work.borrow().size() {
        ctx.push(H, qubits![work.borrow()[i]]);
    }

    for i in 0 .. 2 * n {
        ctx.control(qubits![work.borrow()[i as usize]], true);
        mod_multiplier(ctx, &mult.borrow().clone(), pow_mod(a, 2i32.pow(i), p), p);
        ctx.decontrol(qubits![work.borrow()[i as usize]])
    }

    // inv_qft(ctx, &work.borrow().clone());
    ctx.begin_dagger();
    qft(ctx, &work.borrow().clone());
    ctx.end_dagger();

    ctx.exit();

    ctx.measure(work.borrow().clone());
}

#[derive(Debug, Copy, Clone)]
struct Fraction {
    denominator: i32,
    numerator: i32,
}

impl Fraction {
    pub fn new(denominator: i32, numerator: i32) -> Self {
        let gcd = gcd(denominator, numerator);
        Fraction { denominator: denominator / gcd, numerator: numerator / gcd }
    }
}

fn fraction(frac: &Fraction) -> Vec<i32> {
    let mut p = frac.denominator;
    let mut q = frac.numerator;
    let mut fractions = Vec::<i32>::new();
    while q > 0 {
        fractions.push(p / q);
        (p, q) = (q, p % q);
    }
    [0].into_iter().chain(fractions.into_iter()).collect()
}

fn fraction_val(frac: &[i32]) -> Fraction {
    let mut p = 1;
    let mut q = *frac.last().unwrap();
    for &a in frac[0 .. frac.len() - 1].iter().rev() {
        (q, p) = (a * q + p, q);
    }
    Fraction { denominator: p, numerator: q }
}

/// k: measured state
/// q: 2**num_measured_qubits
///
fn fraction_decompose(k: i32, q: i32, n: i32) -> i32 {
    let fractions = fraction(&Fraction::new(q, k));
    println!("{:?}", fractions);
    let mut last_fraction = Fraction::new(q, k);
    for i in 1 ..= fractions.len() {
        let new_fraction = fraction_val(&fractions[0 .. i]);
        println!("{:?}", new_fraction);
        if last_fraction.denominator >= n {
            return last_fraction.denominator;
        }
        last_fraction = new_fraction;
    }
    last_fraction.denominator
}

fn shor_period_finder(a: i32, p: i32) -> i32 {
    let mut ctx = get_ctx_with_default_passes();
    shor_period_finder_circuit(&mut ctx, a, p);
    println!("circuit length: {}", ctx.circuit.operations.len());
    // for op in ctx.circuit.operations.iter() {
    //     println!("{:?}", op.operation);
    // }
    let instructions = ctx.compile_circuit();
    // print_instructions(&instructions);
    println!("{} instructions", instructions.len());
    let bytecode: ByteCode = instructions.into();
    println!("bytecode length: {}", bytecode.len());
    let result: ExecuteResult = execute_bytecode(bytecode, 10).into();
    ctx.set_measurement_result(result.measurement);
    let result = ctx.get_measurement_result().unwrap();
    println!("result: {:?}", result);

    let mut period = 0;
    let qubits = p.ilog2() + 1;
    for i in 0 .. result.measurements.len() {
        let measurement = &result.measurements[i];
        let value = measurement.value;
        let q = 2i32.pow(qubits * 2);
        println!("q = {}", q);
        println!("value = {}", value);
        let maybe_period = fraction_decompose(value as i32, q, p);
        println!("period = {}", maybe_period);
        period = max(maybe_period, period);
    }

    period
}

#[test]
fn test_shor_period_finder() {
    println!("{}", shor_period_finder(14, 15));
}
