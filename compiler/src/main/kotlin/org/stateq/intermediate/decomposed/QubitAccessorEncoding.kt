package org.stateq.intermediate.decomposed

import org.stateq.expression.IntExpr
import org.stateq.intermediate.DecomposedInstruction
import org.stateq.qubit.QubitAccessor
import org.stateq.compiler.CodeGenerator
import org.stateq.util.Location

class QubitAccessorEncoding(
    val accessor: QubitAccessor,
    val value: IntExpr,
    override val location: Location? = null
) : DecomposedInstruction {
    override fun emit(codegen: CodeGenerator) {
        codegen.qubitAccessorEncode(accessor, value)
    }
}
