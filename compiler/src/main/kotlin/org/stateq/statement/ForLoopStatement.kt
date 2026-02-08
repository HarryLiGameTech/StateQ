package org.stateq.statement

import org.stateq.expression.IterableExpr
import org.stateq.intermediate.decomposible.BasicBlock
import org.stateq.intermediate.decomposible.ForLoop
import org.stateq.parameter.ClassicalVariableBase
import org.stateq.type.ClassicalTrait
import org.stateq.util.Location

class ForLoopStatement<out T: ClassicalTrait>(
    val iterator: ClassicalVariableBase<T>,
    val iterable: IterableExpr<T>,
    val loopBody: StatementsBlock,
    override val location: Location,
) : Statement() {
    override val transpiled: BasicBlock by lazy {
        ForLoop(iterator, iterable, loopBody.transpile(), location).asBasicBlock()
    }
}
