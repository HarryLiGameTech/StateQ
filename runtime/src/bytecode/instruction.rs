use std::fmt::{Debug, Display, Formatter, Write};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use strum_macros::IntoStaticStr;
use crate::bytecode::ByteCode;
use crate::{dispatch, raise_error, use_enum};
use crate::gate::standard::{StandardDoubleGate, StandardSingleGate, StandardTripleGate};
use crate::operation::elementary::ElementaryOperation;
use crate::operation::elementary::standard::StandardOperation;
use crate::operation::{DoubleTargetOperation, DynamicTargetOperation, Operation, SingleTargetOperation, TripleTargetOperation};

#[repr(u8)]
pub enum Instruction {
    Nop,
    Primitive {
        opcode: PrimitiveOpCode,
        params: Vec<InstrParam>,
    },
    StandardGateOperation {
        opcode: StandardOpCode,
        params: Vec<InstrParam>,
        targets: Vec<u32>,
    },
    CustomGateOperation {
        name: [u8; 16],
        params: Vec<InstrParam>,
        targets: Vec<u32>,
    },
}

impl From<Operation> for Instruction {
    fn from(operation: Operation) -> Self {
        use InstrParam::*;
        match operation {
            Operation::Elementary(ElementaryOperation::Standard(operation)) => {
                match operation {
                    StandardOperation::Single(operation) => {
                        use StandardSingleGate::*;
                        let (op_code, params) = match operation.get_gate() {
                            I => (StandardOpCode::I, vec![]),
                            H => (StandardOpCode::H, vec![]),
                            X => (StandardOpCode::X, vec![]),
                            Y => (StandardOpCode::Y, vec![]),
                            Z => (StandardOpCode::Z, vec![]),
                            XPOW { t } => (StandardOpCode::XPOW, vec![Float(t)]),
                            YPOW { t } => (StandardOpCode::YPOW, vec![Float(t)]),
                            ZPOW { t } => (StandardOpCode::ZPOW, vec![Float(t)]),
                            S => (StandardOpCode::S, vec![]),
                            SD => (StandardOpCode::SD, vec![]),
                            T => (StandardOpCode::T, vec![]),
                            TD => (StandardOpCode::TD, vec![]),
                            V => (StandardOpCode::V, vec![]),
                            VD => (StandardOpCode::VD, vec![]),
                            P { angle } => (StandardOpCode::P, vec![Float(angle)]),
                            RX { angle } => (StandardOpCode::RX, vec![Float(angle)]),
                            RY { angle } => (StandardOpCode::RY, vec![Float(angle)]),
                            RZ { angle } => (StandardOpCode::RZ, vec![Float(angle)]),
                            RN { nx, ny, nz, angle } => (
                                StandardOpCode::RN,
                                vec![Float(nx), Float(ny), Float(nz), Float(angle)]
                            ),
                            U { theta, lambda, phi } => (
                                StandardOpCode::U,
                                vec![Float(theta), Float(lambda), Float(phi)]
                            ),
                        };
                        Instruction::StandardGateOperation {
                            opcode: op_code, params, targets: vec![operation.get_target()]
                        }
                    }
                    StandardOperation::Double(operation) => {
                        use StandardDoubleGate::*;
                        let (op_code, params) = match operation.get_gate() {
                            CX => (StandardOpCode::CX, vec![]),
                            CZ => (StandardOpCode::CZ, vec![]),
                            CP { angle } => (StandardOpCode::CP, vec![InstrParam::Float(angle)]),
                            SWP => (StandardOpCode::SWP, vec![]),
                            ISWP => (StandardOpCode::ISWP, vec![]),
                            ISWPD => (StandardOpCode::ISWPD, vec![]),
                            SSWP => (StandardOpCode::SSWP, vec![]),
                            SSWPD => (StandardOpCode::SSWPD, vec![]),
                            SISWP => (StandardOpCode::SISWP, vec![]),
                            SISWPD => (StandardOpCode::SISWPD, vec![]),
                        };
                        let (target0, target1) = operation.get_target();
                        Instruction::StandardGateOperation {
                            opcode: op_code, params, targets: vec![target0, target1]
                        }
                    },
                    StandardOperation::Triple(operation) => {
                        use StandardTripleGate::*;
                        let (op_code, params) = match operation.get_gate() {
                            CCX => (StandardOpCode::CCX, vec![]),
                        };
                        let (target0, target1, target2) = operation.get_target();
                        Instruction::StandardGateOperation {
                            opcode: op_code, params, targets: vec![target0, target1, target2]
                        }
                    },
                }
            }
            Operation::Elementary(ElementaryOperation::Custom(operation)) => {
                let gate = operation.get_gate();
                let ident: [u8; 16] = gate.ident()
                    .chars().map(|c| c as u8)
                    .collect::<Vec<u8>>()
                    .try_into().unwrap();
                Instruction::CustomGateOperation {
                    name: ident,
                    params: gate.get_params().iter().map(|&param| UInt(param)).collect(),
                    targets: operation.get_target().to_vec(),
                }
            }
            Operation::Elementary(ElementaryOperation::Canonical(operation)) => {
                use InstrParam::Float;
                let gate = operation.get_gate();
                let (target0, target1) = operation.get_target();
                Instruction::StandardGateOperation {
                    opcode: StandardOpCode::CAN,
                    params: vec![Float(gate.tx), Float(gate.ty), Float(gate.tz)],
                    targets: vec![target0, target1],
                }
            }
            _ => raise_error! {
                "Only elementary operations can be compiled to instructions"
            }
        }
    }
}

pub enum InstrParam {
    Float(f64),
    Int(i64),
    UInt(u64),
}

impl Into<Vec<u8>> for InstrParam {
    fn into(self) -> Vec<u8> {
        use_enum!(InstrParam);
        dispatch!(self; Float | Int | UInt => |value| value.to_le_bytes().to_vec())
    }
}

#[repr(u8)]
#[derive(Copy, Clone, IntoStaticStr, FromPrimitive)]
pub enum PrimitiveOpCode {
    Alloc = 0x00,
    Reset = 0x01,
    Measure = 0x02,
}

#[repr(u8)]
#[derive(Copy, Clone, IntoStaticStr, FromPrimitive)]
pub enum StandardOpCode {
/*
    # generated by the following python code
    gates = [
        'I', 'H', 'X', 'Y', 'Z', 'XPOW', 'YPOW', 'ZPOW', 'S', 'SD',
        'T', 'TD', 'V', 'VD', 'P', 'RX', 'RY', 'RZ', 'RN', 'U',
        'CX', 'CY', 'CZ', 'CH', 'CP', 'SWP', 'SSWP', 'SSWPD',
        'ISWP', 'ISWPD', 'SISWP', 'SISWPD', 'CAN', 'CCX', 'CSWP'
    ]
    for i, gate in enumerate(gates):
        print(f'{gate:7} = 0x{hex(i)[2:]:>02},')
*/
    I       = 0x00,
    H       = 0x01,
    X       = 0x02,
    Y       = 0x03,
    Z       = 0x04,
    XPOW    = 0x05,
    YPOW    = 0x06,
    ZPOW    = 0x07,
    S       = 0x08,
    SD      = 0x09,
    T       = 0x0a,
    TD      = 0x0b,
    V       = 0x0c,
    VD      = 0x0d,
    P       = 0x0e,
    RX      = 0x0f,
    RY      = 0x10,
    RZ      = 0x11,
    RN      = 0x12,
    U       = 0x13,
    CX      = 0x14,
    CY      = 0x15,
    CZ      = 0x16,
    CH      = 0x17,
    CP      = 0x18,
    SWP     = 0x19,
    SSWP    = 0x1a,
    SSWPD   = 0x1b,
    ISWP    = 0x1c,
    ISWPD   = 0x1d,
    SISWP   = 0x1e,
    SISWPD  = 0x1f,
    CAN     = 0x20,
    CCX     = 0x21,
    CSWP    = 0x22,
}

fn params_to_string(params: &[InstrParam]) -> String {
    let params_str = params.iter().map(|param| {
        use_enum!(InstrParam);
        dispatch!(param; Float | Int | UInt => |param| format!("{}", param))
    }).fold(String::new(), |acc, param| format!("{}, {}", acc, param));
    params_str.trim_start_matches(", ").to_string()
}

impl Display for Instruction {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Nop => formatter.write_str("NOP"),
            Instruction::Primitive { opcode: op_code, params } => {
                let instr_ident: &'static str = op_code.into();
                formatter.write_str(&format!("{} {}", instr_ident, params_to_string(params)))
            }
            Instruction::StandardGateOperation { opcode: op_code, params, targets } => {
                let gate_ident: &'static str = op_code.into();
                if params.is_empty() {
                    formatter.write_str(&format!("{} {:?}", gate_ident, targets))
                } else {
                    formatter.write_str(&format!("{}({}) {:?}", gate_ident, params_to_string(params), targets))
                }
            }
            Instruction::CustomGateOperation { .. } => {
                todo!()
            }
        }
    }
}

impl Debug for Instruction {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, formatter)
    }
}

impl From<Instruction> for Vec<u8> {
    fn from(instruction: Instruction) -> Self {
        let mut bytes: Vec<u8> = vec![];
        bytes.reserve(16);

        match instruction {
            Instruction::Nop { .. } => {
                bytes.push(0x00);
            }

            Instruction::Primitive { opcode: op_code, params } => {
                bytes.push(0x01);
                bytes.push(op_code as u8);
                bytes.push(params.len() as u8);
                for param in params {
                    bytes.append(&mut param.into());
                }
            }

            Instruction::StandardGateOperation { opcode: op_code, params, targets } => {
                bytes.push(0x02);
                bytes.push(op_code as u8);
                bytes.push(params.len() as u8);
                for param in params {
                    bytes.append(&mut param.into());
                }
                bytes.push(targets.len() as u8);
                for target in targets {
                    bytes.append(&mut target.to_le_bytes().to_vec());
                }
            }

            Instruction::CustomGateOperation { name, params, targets } => {
                bytes.push(0x03);
                for i in name {
                    bytes.push(i);
                }
                bytes.push(params.len() as u8);
                for param in params {
                    bytes.append(&mut param.into());
                }
                bytes.push(targets.len() as u8);
                for target in targets {
                    bytes.append(&mut target.to_le_bytes().to_vec());
                }
            }
        }

        bytes
    }
}

impl From<ByteCode> for Vec<Instruction> {
    fn from(bytecode: ByteCode) -> Self {
        Instruction::parse(&bytecode.0)
    }
}

impl Instruction {

    pub fn parse(bytes: &[u8]) -> Vec<Instruction> {

        let mut instructions: Vec<Instruction> = Vec::new();
        let mut buffer = bytes.to_vec();

        while !buffer.is_empty() {
            if buffer[0] == 0x00 {
                buffer.remove(0);
                instructions.push(Instruction::Nop);
            } else if buffer[0] == 0x01 {
                buffer.remove(0);
                instructions.push(Instruction::primitive_parse(&mut buffer));
            } else if buffer[0] == 0x02 {
                buffer.remove(0);
                instructions.push(Instruction::standard_gate_operation_parse(&mut buffer));
            } else if buffer[0] == 0x03 {
                buffer.remove(0);
                instructions.push(Instruction::custom_gate_operation_parse(&mut buffer));
            }
        }

        instructions
    }

    fn primitive_parse(bytes: &mut Vec<u8>) -> Instruction {

        // read op_code
        let op_code = PrimitiveOpCode::from_u8(bytes[0]).unwrap_or_else(|| {
            raise_error!("Invalid opcode {}", bytes[0])
        });

        // read n_params
        let n_params = bytes[1];
        bytes.remove(0);
        bytes.remove(0);

        // read params
        let params = Instruction::params_parse(bytes, n_params);

        Instruction::Primitive {
            opcode: op_code,
            params,
        }
    }

    fn standard_gate_operation_parse(bytes: &mut Vec<u8>) -> Instruction {

        // read op_code
        let op_code = StandardOpCode::from_u8(bytes[0]).unwrap_or_else(|| {
            raise_error!("Invalid opcode {}", bytes[0])
        });

        // read n_params
        let n_params = bytes[1];
        bytes.remove(0);
        bytes.remove(0);

        // read params
        let params = Instruction::params_parse(bytes, n_params);

        let n_targets = bytes[0];
        bytes.remove(0);
        let targets = Instruction::targets_parse(bytes, n_targets);

        Instruction::StandardGateOperation {
            opcode: op_code,
            params,
            targets,
        }
    }

    fn custom_gate_operation_parse(bytes: &mut Vec<u8>) -> Instruction {

        // read name
        let name = Instruction::name_parse(bytes);

        // read n_params
        let n_params = bytes[0];
        bytes.remove(0);

        // read params
        let params = Instruction::params_parse(bytes, n_params);

        let n_targets = bytes[0];
        bytes.remove(0);
        let targets = Instruction::targets_parse(bytes, n_targets);

        Instruction::CustomGateOperation {
            name,
            params,
            targets,
        }
    }


    fn params_parse(bytes_vector: &mut Vec<u8>, n_params: u8) -> Vec<InstrParam> {
        let mut params: Vec<InstrParam> = Vec::new();

        for _ in 0 .. n_params {
            let raw_bytes = <[u8; 8]>::try_from(bytes_vector[0..8].to_vec());
            bytes_vector.drain(0..8);

            let param = InstrParam::UInt(u64::from_be_bytes(raw_bytes.unwrap()));
            params.push(param);
        }

        params
    }

    fn targets_parse(bytes_vector: &mut Vec<u8>, n_targets: u8) -> Vec<u32> {
        let mut targets: Vec<u32> = Vec::new();

        for _ in 0 .. n_targets {
            let raw_bytes = <[u8; 4]>::try_from(bytes_vector[0..4].to_vec());
            bytes_vector.drain(0..4);
            let target = u32::from_be_bytes(raw_bytes.unwrap());
            targets.push(target);
        }

        targets
    }

    fn name_parse(bytes_vector: &mut Vec<u8>) -> [u8; 16] {
        let mut name: [u8; 16] = [0; 16];
        name[..16].copy_from_slice(&bytes_vector[..16]);
        bytes_vector.drain(0..16);
        name
    }
}
