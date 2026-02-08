package org.stateq.intermediate.decomposible

import org.stateq.expression.IntListGenerator
import org.stateq.expression.IterableExpr
import org.stateq.intermediate.DecomposableInstruction
import org.stateq.intermediate.decomposed.DecomposedBasicBlock
import org.stateq.intermediate.decomposed.DecomposedForLoop
import org.stateq.parameter.ClassicalVariableBase
import org.stateq.parameter.IntVariable
import org.stateq.type.ClassicalTrait
import org.stateq.util.Location

class ForLoop<out T: ClassicalTrait>(
    val iterator: ClassicalVariableBase<T>,
    val iterableExpr: IterableExpr<T>,
    val loopBody: BasicBlock,
    override val location: Location?,
): DecomposableInstruction() {
    override val decomposed: DecomposedBasicBlock by lazy {
        DecomposedForLoop(
            iterator, iterableExpr, loopBody.decompose(), location
        ).asDecomposedBasicBlock()
    }
}

fun IntListGenerator.forEach(
    iterator: IntVariable, loopBody: BasicBlock
) = ForLoop(iterator, this, loopBody, this.location)

fun BasicBlock.loopFor(
    iterator: IntVariable, listGenerator: IntListGenerator, location: Location?
) = ForLoop(iterator, listGenerator, this, location)
