package org.stateq.intermediate.decomposed

import org.stateq.exception.unreachable
import org.stateq.expression.IntExpr
import org.stateq.intermediate.DecomposedInstruction
import org.stateq.qubit.*
import org.stateq.compiler.CodeGenerator
import org.stateq.util.Location

class QubitAccessorDeclaration(
    val qubitAccessor: QubitAccessorDeclarable,
    override val location: Location?,
) : DecomposedInstruction {
    val ident: String get() = qubitAccessor.ident
    val size: IntExpr get() = qubitAccessor.size

    override fun emit(codegen: CodeGenerator) {
        when (val accessor = this.qubitAccessor) {
            is QubitAccessorIndexing -> codegen.declareQubitAccessorIndexing(accessor)
            is QubitAccessorSlicing -> codegen.declareQubitAccessorSlicing(accessor)
            is QubitAccessorConcat -> codegen.declareQubitAccessorConcat(accessor)
            is QubitAccessorAlloc -> {
                codegen.declareQubitAccessorAlloc(accessor)
                accessor.init?.also { codegen.qubitAccessorEncode(accessor, it) }
            }
            else -> unreachable()
        }
    }
}
