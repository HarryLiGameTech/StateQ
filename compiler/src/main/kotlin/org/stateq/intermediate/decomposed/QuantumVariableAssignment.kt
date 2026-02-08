package org.stateq.intermediate.decomposed

import org.stateq.intermediate.DecomposedInstruction
import org.stateq.parameter.QuantumVariable
import org.stateq.qubit.QubitAccessor
import org.stateq.compiler.CodeGenerator
import org.stateq.util.Location

class QuantumVariableAssignment(
    val variable: QuantumVariable,
    val value: QubitAccessor,
    override val location: Location?
) : DecomposedInstruction {
    override fun emit(codegen: CodeGenerator) {
        codegen.quantumVariableAssignment(variable.ident, value)
    }
}
