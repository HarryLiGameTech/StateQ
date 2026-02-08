package org.stateq.intermediate.decomposed

import org.stateq.expression.IterableExpr
import org.stateq.intermediate.DecomposedInstruction
import org.stateq.parameter.ClassicalVariableBase
import org.stateq.compiler.CodeGenerator
import org.stateq.type.ClassicalTrait
import org.stateq.util.Location

class DecomposedForLoop<out T: ClassicalTrait>(
    val iterator: ClassicalVariableBase<T>,
    val iterableExpr: IterableExpr<T>,
    val loopBody: DecomposedBasicBlock,
    override val location: Location?,
): DecomposedInstruction {
    override fun emit(codegen: CodeGenerator) {
        codegen.forEachLoop(iterator, iterableExpr) {
            loopBody.emit(codegen)
        }
    }
}
