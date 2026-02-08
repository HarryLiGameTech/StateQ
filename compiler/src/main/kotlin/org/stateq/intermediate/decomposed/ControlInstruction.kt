package org.stateq.intermediate.decomposed

import org.stateq.intermediate.DecomposedInstruction
import org.stateq.qubit.QubitAccessor
import org.stateq.compiler.CodeGenerator
import org.stateq.util.Location

class BeginControl(
    val ctrlQubits: QubitAccessor, val condition: Boolean, override val location: Location?
) : DecomposedInstruction {
    override fun emit(codegen: CodeGenerator) {
        codegen.beginControl(ctrlQubits, condition)
    }
}

class EndControl(
    val ctrlQubits: QubitAccessor, override val location: Location?
) : DecomposedInstruction {
    override fun emit(codegen: CodeGenerator) {
        codegen.endControl(ctrlQubits)
    }
}
