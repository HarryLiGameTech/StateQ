package org.stateq.intermediate.decomposed

import org.stateq.compiler.CodeGenerator
import org.stateq.intermediate.DecomposedInstruction
import org.stateq.qubit.QubitAccessor
import org.stateq.util.Location

class Measurement(
    val target: QubitAccessor, override val location: Location?
) : DecomposedInstruction {
    override fun emit(codegen: CodeGenerator) {
        codegen.measure(target)
    }
}
