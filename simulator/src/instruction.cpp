#include "instruction.hpp"

void StandardGateInstruction::execute(Qureg* qureg) const
{
    switch (this->gate) {
        case StandardGate::I:
            break;
        case StandardGate::H:
            assertNumParamsAndTargetSize(0, 1);
            hadamard(*qureg, (int) targets[0]);
            break;
        case StandardGate::X:
            assertNumParamsAndTargetSize(0, 1);
            pauliX(*qureg, (int) targets[0]);
            break;
        case StandardGate::Y:
            assertNumParamsAndTargetSize(0, 1);
            pauliY(*qureg, (int) targets[0]);
            break;
        case StandardGate::Z:
            assertNumParamsAndTargetSize(0, 1);
            pauliZ(*qureg, (int) targets[0]);
            break;
        case StandardGate::S:
            assertNumParamsAndTargetSize(0, 1);
            sGate(*qureg, (int) targets[0]);
            break;
        case StandardGate::SD:
            assertNumParamsAndTargetSize(0, 1);
            rotateX(*qureg, (int) targets[0], -M_PI / 2);
            break;
        case StandardGate::T:
            assertNumParamsAndTargetSize(0, 1);
            tGate(*qureg, (int) targets[0]);
            break;
        case StandardGate::TD:
            assertNumParamsAndTargetSize(0, 1);
            rotateX(*qureg, (int) targets[0], -M_PI / 4);
            break;
        case StandardGate::P:
            assertNumParamsAndTargetSize(1, 1);
            phaseShift(*qureg, (int) targets[0], params[0].float64);
            break;
        case StandardGate::RX:
            assertNumParamsAndTargetSize(1, 1);
            rotateX(*qureg, (int) targets[0], params[0].float64);
            break;
        case StandardGate::RY:
            assertNumParamsAndTargetSize(1, 1);
            rotateY(*qureg, (int) targets[0], params[0].float64);
            break;
        case StandardGate::RZ:
            assertNumParamsAndTargetSize(1, 1);
            rotateZ(*qureg, (int) targets[0], params[0].float64);
            break;
        case StandardGate::CX:
            assertNumParamsAndTargetSize(0, 2);
            controlledNot(*qureg, (int) targets[0], (int) targets[1]);
            break;
        case StandardGate::CY:
            assertNumParamsAndTargetSize(0, 2);
            controlledPauliY(*qureg, (int) targets[0], (int) targets[1]);
            break;
        case StandardGate::CZ:
            assertNumParamsAndTargetSize(0, 2);
            controlledRotateZ(*qureg, (int) targets[0], (int) targets[1], 0);
            break;
        case StandardGate::CP:
            assertNumParamsAndTargetSize(1, 2);
            controlledPhaseShift(*qureg, (int) targets[0], (int) targets[1], params[0].float64);
            break;
        case StandardGate::SWP:
            assertNumParamsAndTargetSize(0, 2);
            swapGate(*qureg, (int) targets[0], (int) targets[1]);
            break;
        case StandardGate::SSWP:
            assertNumParamsAndTargetSize(0, 2);
            sqrtSwapGate(*qureg, (int) targets[0], (int) targets[1]);
            break;
        case StandardGate::CCX:
            assertNumParamsAndTargetSize(0, 3);
            multiControlledMultiQubitNot(*qureg, (int*) targets.data(), 2, (int*) targets.data() + 2, 1);
            break;
        default:
            throw UnsupportedGateException(this->gateIdent());
    }
}
